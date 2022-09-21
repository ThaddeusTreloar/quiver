use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum HandlerType {
    All,
    Calendar,
} 

