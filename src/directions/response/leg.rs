//! A single leg consisting of a set of steps in a DirectionsResult. Some fields
//! in the leg may not be returned for all requests.

use crate::{
    directions::response::{
        directions_distance::DirectionsDistance,
        directions_duration::DirectionsDuration,
        step::Step,
        transit_time::TransitTime,
    }, // crate::directions::response
    latlng::LatLng,
}; // use crate
use serde::Deserialize;

/// A single leg consisting of a set of steps in a DirectionsResult. Some fields
/// in the leg may not be returned for all requests. Note that though this
/// result is "JSON-like," it is not strictly JSON, as it directly and
/// indirectly includes LatLng objects.

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct Leg {
    /// An estimated arrival time for this leg. Only applicable for
    /// `TravelMode::Transit` requests.
    pub arrival_time: Option<TransitTime>,

    /// An estimated departure time for this leg. Only applicable for
    /// `TravelMode::Transit` requests.
    pub departure_time: Option<TransitTime>,

    /// The total distance covered by this leg. This property may be undefined
    /// as the distance may be unknown.
    pub distance: DirectionsDistance,

    /// The total duration of this leg. This property may be undefined as the
    /// duration may be unknown.
    pub duration: DirectionsDuration,

    /// The total duration of this leg, taking into account the traffic
    /// conditions indicated by the `with_traffic_model()` method. This property
    /// may be undefined as the duration may be unknown. Only available to
    /// Premium Plan customers.
    pub duration_in_traffic: Option<DirectionsDuration>,

    /// The address of the destination of this leg.
    pub end_address: String,

    /// The Directions Service calculates directions between locations by using
    /// the nearest transportation option (usually a road) at the start and end
    /// locations. `end_location` indicates the actual geocoded destination,
    /// which may be different than the end_location of the last step if, for
    /// example, the road is not near the destination of this leg.
    pub end_location: LatLng,

    /// The address of the origin of this leg.
    pub start_address: String,

    /// The Directions Service calculates directions between locations by using
    /// the nearest transportation option (usually a road) at the start and end
    /// locations. `start_location` indicates the actual geocoded origin, which
    /// may be different than the `start_location` of the first step if, for
    /// example, the road is not near the origin of this leg.
    pub start_location: LatLng,

    /// An array of `Steps`, each of which contains information about the
    /// individual steps in this leg.
    pub steps: Vec<Step>,
} // struct

impl Leg {

    /// A helper function for destructuring (or serializing) the optional
    /// `duration_in_traffic` field. If the `Duration` struct is populated, this
    /// function will return the _text_ field as a `String`. If the _Duration_
    /// struct is empty, this function will return `None`.
    /// ```rust
    /// let duration_in_traffic_text = leg.get_duration_in_traffic_text();
    /// ```
    pub fn get_duration_in_traffic_text(&self) -> Option<&String> {
        match &self.duration_in_traffic {
            Some(duration) => Some(&duration.text),
            None => None,
        } // match
    } // fn

    /// A helper function for destructuring (or serializing) the optional
    /// `duration_in_traffic` field. If the `Duration` struct is populated, this
    /// function will return the _value_ field as a `time::Duration`. If the
    /// _Duration_ struct is empty, this function will return `None`.
    /// ```rust
    /// let duration_in_traffic_value = leg.get_duration_in_traffic_value();
    /// ```
    pub fn get_duration_in_traffic_value(&self) -> Option<i64> {
        match &self.duration_in_traffic {
            Some(duration) => Some(duration.value.num_seconds()),
            None => None,
        } // match
    } // fn

    /// A helper function for destructuring (or serializing) the optional
    /// `arrival_time` field. If the `Time` struct is populated, this function
    /// will return the _text_ field as a `String`. If the _Time_ struct is
    /// empty, this function will return `None`.
    /// ```rust
    /// let arrival_time_text = leg.get_arrival_time_text();
    /// ```
    pub fn get_arrival_time_text(&self) -> Option<&String> {
        match &self.arrival_time {
            Some(time) => Some(&time.text),
            None => None,
        } // match
    } // fn

    /// A helper function for destructuring (or serializing) the optional
    /// `arrival_time` field. If the `Time` struct is populated, this function
    /// will return the _time_zone_ field as a `String` type. If the _Time_
    /// struct is empty, this function will return `None`.
    /// ```rust
    /// let arrival_time_zone = leg.arrival_time_zone();
    /// ```
    pub fn get_arrival_time_value(&self) -> Option<i64> {
        match &self.arrival_time {
            Some(time) => Some(time.value.timestamp()),
            None => None,
        } // match
    } // fn

    /// A helper function for destructuring (or serializing) the optional
    /// `arrival_time` field. If the `Time` struct is populated, this function
    /// will return the _time_ field as a `i64` UNIX timestamp. If the _Time_
    /// struct is empty, this function will return `None`.
    /// ```rust
    /// let arrival_time_zone = leg.arrival_time_zone();
    /// ```
    pub fn get_arrival_time_zone(&self) -> Option<String> {
        match &self.arrival_time {
            Some(time) => Some(time.time_zone.name().to_string()),
            None => None,
        } // match
    } // fn

    /// A helper function for destructuring (or serializing) the optional
    /// `departure_time` field. If the `Time` struct is populated, this function
    /// will return the _text_ field as a `String`. If the _Time_ struct is
    /// empty, this function will return `None`.
    /// ```rust
    /// let departure_time_text = leg.get_departure_time_text();
    /// ```
    pub fn get_departure_time_text(&self) -> Option<&String> {
        match &self.departure_time {
            Some(time) => Some(&time.text),
            None => None,
        } // match
    } // fn

    /// A helper function for destructuring (or serializing) the optional
    /// `departure_time` field. If the `Time` struct is populated, this function
    /// will return the _time_zone_ field as a `String` type. If the _Time_
    /// struct is empty, this function will return `None`.
    /// ```rust
    /// let departure_time_zone = leg.departure_time_zone();
    /// ```
    pub fn get_departure_time_value(&self) -> Option<i64> {
        match &self.departure_time {
            Some(time) => Some(time.value.timestamp()),
            None => None,
        } // match
    } // fn

    /// A helper function for destructuring (or serializing) the optional
    /// `departure_time` field. If the `Time` struct is populated, this function
    /// will return the _time_ field as a `i64` UNIX timestamp. If the _Time_
    /// struct is empty, this function will return `None`.
    /// ```rust
    /// let departure_time_zone = leg.departure_time_zone();
    /// ```
    pub fn get_departure_time_zone(&self) -> Option<String> {
        match &self.departure_time {
            Some(time) => Some(time.time_zone.name().to_string()),
            None => None,
        } // match
    } // fn

} // impl