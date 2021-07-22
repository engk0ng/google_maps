use crate::geocoding::{
    OUTPUT_FORMAT,
    SERVICE_URL,
    reverse::ReverseRequest,
}; // crate::geocoding

impl<'a> ReverseRequest<'a> {

    /// Returns the URL query string that represents the query you've built.
    ///
    /// ## Description:
    ///
    /// Returns the query string that will be sent to the Google Maps API. It
    /// is the result of the builder pattern. This method could be useful for
    /// records or logging. It could also be used for passing to your HTTP
    /// client of choice and executing the HTTP GET request yourself.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn url_string(&'a mut self) -> String {
        match &self.query {
            // If query string has already been built, return it:
            Some(url_string) => format!("{}/{}?{}", SERVICE_URL, OUTPUT_FORMAT, url_string),
            // If it hasn't been built, build it:
            None => self.build().query.as_ref().unwrap().clone(),
        } // match
    } // fn

} // impl