extern crate confy;
use crate::handler::lib::handler_enums::HandlerTypes;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreConfig{
    pub active_handlers: Box<[HandlerTypes]>,
    pub persistent_config: bool,
}

impl ::std::default::Default for CoreConfig {
    fn default() -> Self {
        Self {
            active_handlers: Box::new([HandlerTypes::All]),
            persistent_config: true,
        }
    }
}

