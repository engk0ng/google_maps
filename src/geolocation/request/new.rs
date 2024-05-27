use crate::{geolocation::request::Request, GoogleMapsClient};

use super::wifi_access_point::WiFiAccessPoint;

impl<'a> Request<'a> {
    #[must_use]
    pub const fn new(
        client: &GoogleMapsClient,
        key: String,
        wifi_access_points: Vec<WiFiAccessPoint>
    ) -> Request {
        Request {
            client,
            key,
            wifi_access_points,
            query: None,
            payload: serde_json::Value::Null
        }
    }
}