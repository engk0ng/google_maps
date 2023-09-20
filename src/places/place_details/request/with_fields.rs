use crate::places::place_details::Field;
use crate::places::place_details::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {

    /// Adds the requested fields to the Places API _Place Details_ query.
    ///
    /// ## Arguments:
    ///
    /// * `fields` ‧ Use the fields parameter to specify a comma-separated list
    /// of place data types to return. For example:
    /// `fields=formatted_address,name,geometry`. Use a forward slash when
    /// specifying compound values. For example: `opening_hours/open_now`.
    ///
    /// Fields are divided into three billing categories: Basic, Contact, and
    /// Atmosphere. Basic fields are billed at base rate, and incur no
    /// additional charges. Contact and Atmosphere fields are billed at a higher
    /// rate. See the [pricing sheet](https://cloud.google.com/maps-platform/pricing/sheet/)
    /// for more information. Attributions, `html_attributions`, are always
    /// returned with every call, regardless of whether the field has been
    /// requested.
    ///
    /// * Caution: Place Search requests and Place Details requests do not
    /// return the same fields. Place Search requests return a subset of the
    /// fields that are returned by Place Details requests. If the field you
    /// want is not returned by Place Search, you can use Place Search to get a
    /// `place_id`, then use that Place ID to make a Place Details request. For
    /// more information on the fields that are unavailable in a Place Search
    /// request, see
    /// [Places API fields support](https://developers.google.com/maps/documentation/places/web-service/place-data-fields#places-api-fields-support).
    ///
    /// * Warning: If you do not specify at least one field with a request, or
    /// if you omit the `fields` parameter from a request, ALL possible fields
    /// will be returned, and you will be billed accordingly. This applies only
    /// to Place Details requests.

    pub fn with_fields(&'a mut self, fields: Vec<Field>) -> &'a mut Request {
        // Set fields in Request struct.
        self.fields = Some(fields);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl