//! Put `use google_maps::prelude::*;` in your code will to get more convenient
//! access to everything you need. If you're not concerned with name space
//! collisions or conflicts, you can glob import all `google_maps` structs and
//! enums by using this module.

// Re-export dependencies

#[cfg(any(
    feature = "directions",
    feature = "distance_matrix",
    feature = "time_zone"
))]
pub use chrono::{offset::TimeZone, DateTime, Duration, Local, NaiveDate, NaiveDateTime, Utc};
#[cfg(any(feature = "directions", feature = "time_zone"))]
pub use chrono_tz::Tz;
pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;

// =============================================================================
//
// Common structures:

pub use crate::{
    client::GoogleMapsClient as ClientSettings, client::GoogleMapsClient,
    error::Error as GoogleMapsError, error::Error, types::error::Error as TypeError,
}; // crate

#[cfg(any(feature = "geocoding", feature = "places"))]
pub use crate::types::address_component::AddressComponent;
#[cfg(any(
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places",
    feature = "geolocation"
))]
pub use crate::types::bounds::Bounds;
#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "geocoding",
    feature = "geolocation"
))]
pub use crate::types::country::Country;
#[cfg(any(feature = "geocoding", feature = "places"))]
pub use crate::types::geometry::Geometry;
#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places",
    feature = "time_zone"
))]
pub use crate::types::language::Language;
#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "elevation",
    feature = "geocoding",
    feature = "places",
    feature = "roads",
    feature = "time_zone"
))]
pub use crate::types::latlng::LatLng;
#[cfg(any(feature = "geocoding", feature = "places"))]
pub use crate::types::location_type::LocationType;
#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
pub use crate::types::place_type::PlaceType;
#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
pub use crate::types::region::Region;

// =============================================================================
//
// Optional structures:

// -----------------------------------------------------------------------------

#[cfg(feature = "enable-reqwest")]
pub use crate::request_rate::api::Api;

// -----------------------------------------------------------------------------

#[cfg(any(feature = "directions", feature = "distance_matrix"))]
pub use crate::directions::{
    request::{
        avoid::Avoid, departure_time::DepartureTime, location::Location,
        traffic_model::TrafficModel, transit_mode::TransitMode,
        transit_route_preference::TransitRoutePreference, unit_system::UnitSystem,
        waypoint::Waypoint,
    }, // request
    response::{
        driving_maneuver::DrivingManeuver, leg::Leg, overview_polyline::OverviewPolyline,
        polyline::Polyline, route::Route, status::Status as DirectionsStatus, step::Step,
        transit_agency::TransitAgency, transit_currency::TransitCurrency,
        transit_details::TransitDetails, transit_fare::TransitFare, transit_line::TransitLine,
        transit_stop::TransitStop, transit_time::TransitTime, transit_vehicle::TransitVehicle,
        Response as DirectionsResponse,
    }, // response
    travel_mode::TravelMode,
    vehicle_type::VehicleType,
}; // crate::directions

// -----------------------------------------------------------------------------

#[cfg(feature = "directions")]
pub use crate::directions::{
    error::Error as DirectionsError,
    request::Request as DirectionsRequest,
    response::{directions_distance::DirectionsDistance, directions_duration::DirectionsDuration}, // response
}; // crate::directions

// -----------------------------------------------------------------------------

#[cfg(feature = "distance_matrix")]
pub use crate::distance_matrix::{
    error::Error as DistanceMatrixError, request::Request as DistanceMatrixRequest,
    response::status::Status as DistanceMatrixStatus, response::Response as DistanceMatrixResponse,
}; // use crate::distance_matrix

// -----------------------------------------------------------------------------

#[cfg(feature = "elevation")]
pub use crate::elevation::{
    error::Error as ElevationError,
    request::{locations::Locations as ElevationLocations, Request as ElevationRequest}, // request
    response::{point::Point, status::Status as ElevationStatus, Response as ElevationResponse}, // response
}; // crate::elevation

// -----------------------------------------------------------------------------

#[cfg(feature = "geocoding")]
pub use crate::geocoding::{
    error::Error as GeocodingError,
    forward::{component::Component as GeocodingComponent, ForwardRequest as GeocodingRequest}, // forward
    response::{
        geocoding::Geocoding, plus_code::PlusCode, status::Status as GeocodingStatus,
        Response as GeocodingResponse,
    }, // response
    reverse::ReverseRequest as GeocodingReverseRequest,
}; // crate::geocoding

// -----------------------------------------------------------------------------

#[cfg(feature = "time_zone")]
pub use crate::time_zone::{
    error::Error as TimeZoneError,
    request::Request as TimeZoneRequest,
    response::{status::Status as TimeZoneStatus, Response as TimeZoneResponse}, // reponse
}; // crate::time_zone

#[cfg(feature = "geolocation")]
pub use crate::geolocation::{
    error::Error as GeolocationError,
    request::Request as GeolocationRequest,
    response::{status::Status as GeolocationStatus, Response as GeolocationResponse}
};

// -----------------------------------------------------------------------------

#[cfg(feature = "autocomplete")]
pub use crate::places::place_autocomplete::{
    error::Error as AutocompleteError,
    request::{autocomplete_type::AutocompleteType, Request as PlaceAutocompleteRequest}, // request
    response::{
        matched_substring::MatchedSubstring, prediction::Prediction,
        status::Status as AutocompleteStatus, structured_format::StructuredFormat, term::Term,
        Response as AutocompleteResponse,
    }, // response
}; // crate::places::place_autocomplete

// -----------------------------------------------------------------------------

#[cfg(feature = "autocomplete")]
pub use crate::places::place_autocomplete::request::Request as QueryAutocompleteRequest; // crate::places::place_autocomplete

// -----------------------------------------------------------------------------

#[cfg(feature = "places")]
pub use crate::places::{
    business_status::BusinessStatus, error::Error as PlacesError, place::Place,
    place_editorial_summary::PlaceEditorialSummary, place_opening_hours::PlaceOpeningHours,
    place_opening_hours_period::PlaceOpeningHoursPeriod,
    place_opening_hours_period_detail::PlaceOpeningHoursPeriodDetail, place_photo::PlacePhoto,
    place_review::PlaceReview, place_special_day::PlaceSpecialDay,
    secondary_hours_type::SecondaryHoursType, status::Status as PlacesStatus,
}; // crate::places

// -----------------------------------------------------------------------------

#[cfg(feature = "places")]
pub use crate::places::place_search::text_search::{
    request::Request as TextSearchRequest, response::Response as TextSearchResponse,
}; // text_search

// -----------------------------------------------------------------------------

#[cfg(feature = "places")]
pub use crate::places::place_details::{
    field::Field, request::Request as PlaceDetailsRequest,
    response::Response as PlaceDetailsResponse, sort_order::SortOrder,
}; // place_details

// -----------------------------------------------------------------------------

#[cfg(feature = "roads")]
pub use crate::roads::{
    error::Error as RoadsError, error_response::ErrorResponse as RoadsErrorResponse,
    snapped_point::SnappedPoint, status::Status as RoadsStatus,
}; // crate::roads

// -----------------------------------------------------------------------------

#[cfg(feature = "roads")]
pub use crate::roads::snap_to_roads::{
    request::Request as SnapToRoadsRequest, response::Response as SnapToRoadsResponse,
}; // crate::roads::snap_to_roads

// -----------------------------------------------------------------------------

#[cfg(feature = "roads")]
pub use crate::roads::nearest_roads::{
    request::Request as NearestRoadsRequest, response::Response as NearestRoadsResponse,
}; // crate::roads::nearest_roads
