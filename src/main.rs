mod core;
mod security;
mod handler;
mod shared;
use env_logger;
mod client;

fn main() {
    env_logger::init();
    let config: core::config::CoreConfig = core::init::init();
    match core::main::main(config) {
        Ok(_val) => return,
        Err(_err) => unimplemented!(),
    }
}
