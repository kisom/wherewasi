/// core defines the core data model (the coordinates type)
/// and functions for interacting with the model.
extern crate serde;
extern crate serde_json;
extern crate time;

use std::fmt;

/// Coordinates is presumed to be a WGS84 position paired with a UTC timestamp
/// and an altitude. Both the ground position and altitude have a corresponding
/// accuracy measurement.
#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinates {
    // A set of coordinates in 3D.
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,

    // A coordinate in time.
    pub timestamp: u64,
    
    // 3D accuracy.
    pub gacc: f64,   // ground accuracy
    pub aacc: f64    // altitude accuracy
}

impl Coordinates {
    pub fn new(lat: f64, lon: f64, alt: f64, gacc: f64, aacc: f64) -> Coordinates {
        let timestamp = time::get_time();

        Coordinates {
            lat: lat,
            lon: lon,
            alt: alt,
            gacc: gacc,
            aacc: aacc,
            timestamp: (timestamp.sec as u64 * 1000) + (timestamp.nsec as u64 / 1000000),
        }
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}º", self.lat);
        if self.lat >= 0.0 {
            write!(f, "N")
        } else {
            write!(f, "S")
        }
    }
}
