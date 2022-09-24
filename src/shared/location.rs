use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinate {
    pub latitude: f64,
    pub lontitude: f64,
    pub altitude: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub common_name: String,
    pub coordinate: Coordinate,
    pub address: String,
}