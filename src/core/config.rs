extern crate confy;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreConfig{
    active_handlers: Box<[String]>,

}

impl ::std::default::Default for CoreConfig {
    fn default() -> Self{
        Self {
            active_handlers: Box::new(["*".to_owned()])
        }
    }
}

