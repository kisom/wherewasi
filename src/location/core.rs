/// core defines the core data model (the coordinates type)
/// and functions for interacting with the model.
extern crate serde;
extern crate serde_json;
extern crate time;

use std::fmt;

fn north_or_south(latitude: f64) -> String {
    if latitude >= 0.0 {
        "N".to_string()
    } else {
        "S".to_string()
    }
}

fn east_or_west(longitude: f64) -> String {
    if longitude >= 0.0 {
        "E".to_string()
    } else {
        "W".to_string()
    }
}

/// Coordinates is presumed to be a WGS84 position paired with a UTC timestamp
/// and an altitude. Both the ground position and altitude have a corresponding
/// accuracy measurement.
#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinates {
    // A set of coordinates in 3D.
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,

    // A coordinate in time.
    pub timestamp: u64,
    
    // 3D accuracy.
    pub gacc: f64,   // ground accuracy
    pub aacc: f64    // altitude accuracy
}

impl Coordinates {
    pub fn new(latitude: f64, longitude: f64, altitude: f64, gacc: f64, aacc: f64) -> Coordinates {
        let timestamp = time::get_time();

        Coordinates {
            latitude: latitude,
            longitude: longitude,
            altitude: altitude,
            gacc: gacc,
            aacc: aacc,
            timestamp: (timestamp.sec as u64 * 1000) + (timestamp.nsec as u64 / 1000000),
        }
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}°{}, {}°{} within {}m @ {}m within {}m", 
            self.latitude.abs(), north_or_south(self.latitude),
            self.longitude.abs(), east_or_west(self.longitude),
            self.gacc,
            self.altitude, self.aacc)
    }
}
