
use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Get,
    Put,
    Pop,
    Edit,
}