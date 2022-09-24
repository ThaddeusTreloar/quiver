extern crate confy;
use crate::shared::lib::request::HandlerType;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreConfig{
    pub active_handlers: Vec<HandlerType>,
    pub persistent_config: bool,
}

impl ::std::default::Default for CoreConfig {
    fn default() -> Self {
        Self {
            active_handlers: vec!(HandlerType::All),
            persistent_config: true,
        }
    }
}

