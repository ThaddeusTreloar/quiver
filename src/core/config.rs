extern crate confy;
use crate::shared::lib::HandlerType;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreConfig{
    pub active_handlers: Vec<HandlerType>,
    pub persistent_config: bool,
    pub core_db_path: String,
}

// todo: check that this lifetime is valid
impl ::std::default::Default for CoreConfig {
    fn default() -> Self {
        Self {
            active_handlers: vec!(HandlerType::All),
            persistent_config: true,
            core_db_path: "./run/core.sqlite".to_owned()
        }
    }
}

