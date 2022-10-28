// Internal
use crate::shared::lib::{
    HandlerType,
    
};
use crate::handler::*;
use crate::core::{
    lib::build_connection_pool,
    config::CoreConfig,
};

// External
use std::{
    thread,
    io::prelude::*, 
};
use failure::Error;
