use crate::error::Error as GoogleMapsError;
use crate::geocoding::{
    response::Response as GeocodingResponse, reverse::ReverseRequest as GeocodingReverseRequest,
}; // use crate::geocoding

// =============================================================================

impl<'a> GeocodingReverseRequest<'a> {
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

    pub async fn execute(&'a mut self) -> Result<GeocodingResponse, GoogleMapsError> {
        self.build().get().await
    } // fn
} // impl
