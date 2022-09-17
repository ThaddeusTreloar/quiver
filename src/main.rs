mod core;
mod security;
mod handler;

fn main() {
    let config: core::config::CoreConfig = core::init::init();
    match core::main::main(config) {
        Ok(_val) => unimplemented!(),
        Err(_err) => unimplemented!(),
    }
}
