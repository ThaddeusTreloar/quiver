extern crate confy;
use crate::handler::lib::handler_enums::HandlerType;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreConfig{
    pub active_handlers: Box<[HandlerType]>,
    pub persistent_config: bool,
}

impl ::std::default::Default for CoreConfig {
    fn default() -> Self {
        Self {
            active_handlers: Box::new([HandlerType::All]),
            persistent_config: true,
        }
    }
}

