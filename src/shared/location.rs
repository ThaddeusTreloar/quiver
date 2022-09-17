use serde::{
    Serialize,
    Deserialize,
};

#[derive(Serialize, Deserialize)]
pub struct Coordinate {
    latitude: f64,
    lontitude: f64,
    altitude: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    common_name: String,
    coordinate: Coordinate,
    address: String,
}