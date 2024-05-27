mod cell_tower;
mod radio_type;
pub mod wifi_access_point;
mod new;
#[cfg(feature = "enable-reqwest")]
mod post;
mod build;
#[cfg(feature = "enable-reqwest")]
mod execute;

use crate::GoogleMapsClient;
use crate::geolocation::request::wifi_access_point::WiFiAccessPoint;


/// The request body must be formatted as JSON. All fields are optional.

#[derive(Debug)]
pub struct Request<'a> {
    client: &'a GoogleMapsClient,
    // Required parameters:
    // --------------------

    /// Your application's API key. This key identifies your application for
    /// purposes of quota management. Learn how to
    /// [get a key](https://developers.google.com/maps/documentation/timezone/get-api-key).
    key: String,

    /// An array of WiFi access point objects. See the [WiFi Access Point
    /// Objects](https://developers.google.com/maps/documentation/geolocation/intro#wifi_access_point_object)
    /// section.
    pub wifi_access_points: Vec<WiFiAccessPoint>,

    // Internal use only:

    query: Option<String>,

    payload: serde_json::Value
} // struct