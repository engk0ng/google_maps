use crate::error::Error as GoogleMapsError;
use crate::geolocation::{
    request::Request as GeolocationRequest, response::Response as GeolocationResponse,
}; // crate::time_zone

// =============================================================================

impl<'a> GeolocationRequest<'a> {
    // -------------------------------------------------------------------------
    //
    /// Executes the query you've built.
    ///
    /// ## Description
    ///
    /// My adventures in Rust became messy so I had to make this method. It
    /// wraps the `.validate()?.build()?.get()?` chain needed at the end of the
    /// builder pattern.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.

    pub async fn execute(&'a mut self) -> Result<GeolocationResponse, GoogleMapsError> {
        self.build().post().await
    } // fn
}