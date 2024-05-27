use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct WiFiAccessPoint {
    // Required parameters:
    // --------------------

    /// (required) The MAC address of the WiFi node. It's typically called a
    /// BSS, BSSID or MAC address. Separators must be `:` (colon).
    #[serde(alias = "macAddress")]
    pub mac_address: String,

    // Optional parameters:
    // --------------------

    /// The current signal strength measured in dBm.
    #[serde(alias = "signalStrength")]
    pub signal_strength: Option<i16>,

    /// The number of milliseconds since this access point was detected.
    #[serde(alias = "age")]
    pub age: Option<u16>,

    /// The channel over which the client is communicating with the access point.
    #[serde(alias = "channel")]
    pub channel: Option<u16>,

    /// The current signal to noise ratio measured in dB.
    #[serde(alias = "signalToNoiseRatio")]
    pub signal_to_noise_ratio: Option<i16>,
} // struct

// -----------------------------------------------------------------------------

impl std::convert::From<&Self> for WiFiAccessPoint {
    /// Converts a borrowed `&LatLng` enum into an owned `LatLng` enum by
    /// copying it.
    fn from(loc: &Self) -> Self {
        (*loc).clone()
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&WiFiAccessPoint> for String {
    /// Converts a borrowed `&LatLng` struct to a `String` that contains a
    /// latitude/longitude pair.
    fn from(point: &WiFiAccessPoint) -> Self {
        format!(
            "{mac_address},{signal_strength},{age},{channel},{signal_to_noise_ratio}",
            mac_address = point.mac_address,
            signal_strength = point.signal_strength.unwrap(),
            age = point.age.unwrap(),
            channel = point.channel.unwrap(),
            signal_to_noise_ratio = point.signal_to_noise_ratio.unwrap()
        ) // format!
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<WiFiAccessPoint> for String {
    /// Converts an owned `LatLng` struct to a `String` that contains a
    /// latitude/longitude pair.
    fn from(poin: WiFiAccessPoint) -> Self {
        Self::from(&poin)
    }
} // impl
