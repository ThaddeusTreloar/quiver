
use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum HandlerTypes {
    All,
    Calendar,
} 

