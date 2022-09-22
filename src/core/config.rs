extern crate confy;
use crate::handler::lib::handler_enums::HandlerType;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreConfig{
    pub active_handlers: Vec<HandlerType>,
    pub persistent_config: bool,
    pub sockets_path: String
}

impl ::std::default::Default for CoreConfig {
    fn default() -> Self {
        Self {
            active_handlers: vec!(HandlerType::All),
            persistent_config: true,
            sockets_path: "/tmp/".to_owned(),
        }
    }
}

