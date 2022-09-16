use crate::handler::lib::enums::HandlerTypes;

#[derive(Debug)]
pub struct CoreSettings {
    handler: Box<[HandlerTypes]>,
}