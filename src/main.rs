mod core;
mod security;
mod handler;
use env_logger;

fn main() {
    env_logger::init();
    let config: core::config::CoreConfig = core::init::init();
    match core::main::main(config) {
        Ok(_val) => unimplemented!(),
        Err(_err) => unimplemented!(),
    }
}
