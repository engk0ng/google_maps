use crate::error::Error as GoogleMapsError;
use crate::request_rate::api::Api;
use crate::geolocation::{
    error::Error as GeolocationError, request::Request as GeolocationRequest,
    response::Response as GeolocationResponse,
    SERVICE_URL
};

use backoff::future::retry;
use backoff::Error::{Permanent, Transient};
use backoff::ExponentialBackoff;

impl<'a> GeolocationRequest<'a> {
    #[tracing::instrument(level = "debug", name = "google_maps.geolocation", skip(self))]
    pub async fn post(&mut self) -> Result<GeolocationResponse, GoogleMapsError> {
        let mut url = format!("{}?", SERVICE_URL);
        match &self.query {
            Some(query) => url.push_str(query.as_str()),
            None => return Err(GeolocationError::QueryNotBuilt).unwrap(),
        }

        self.client
        .rate_limit
        .limit_apis(vec![&Api::All, &Api::Geolocation])
        .await;

        tracing::debug!("Making HTTP POST request to Google Maps Geolocation API: `{url}`");

        let response = retry(ExponentialBackoff::default(), || async {
            let response = self.client.post_request(&url, &self.payload).await;
            match response {
                Ok(response) => {
                    if response.status().is_success() {
                        let text = &response.text().await;
                        match text {
                            Ok(text) => {
                                match serde_json::from_str::<GeolocationResponse>(text) {
                                    Ok(deserialized) => {
                                        Ok(deserialized)
                                    }   
                                    Err(error) => {
                                        tracing::error!("JSON parsing error: {}", error);
                                        Err(Permanent(GeolocationError::SerdeJson(error)))
                                    }
                                }
                            }
                            Err(error) => {
                                tracing::error!("HTTP client returned: {}", error);
                                Err(Permanent(GeolocationError::ReqwestMessage(error.to_string())))
                            }
                        }
                    }
                    else if response.status().is_server_error() || response.status() == 420 {
                        tracing::warn!("HTTP client returned: {}", response.status());
                        Err(Transient {
                            err: GeolocationError::HttpUnsuccessful(response.status().to_string()),
                            retry_after: None,
                        })
                    }
                    else {
                        tracing::error!("HTTP client returned: {}", response.status());
                        Err(Permanent(GeolocationError::HttpUnsuccessful(
                            response.status().to_string(),
                        )))
                    }
                }
                Err(error) => {
                    tracing::warn!("HTTP client returned: {}", error);
                    Err(Transient {
                        err: GeolocationError::Reqwest(error),
                        retry_after: None,
                    })
                }
            }
        }).await.unwrap();

        Ok(response)
    }
}