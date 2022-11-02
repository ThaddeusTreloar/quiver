// Internal 

// External
use confy;
use std::default::Default;
use std::io::{
    prelude::Read,
    BufReader
};
use std::fs::{
    remove_file,
    File
};
use crate::shared::lib::HandlerType;
use serde::{Serialize, Deserialize};
use openssl::{
    pkey::{
        PKey,
        Private
    }
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreConfig{
    pub active_handlers: Vec<HandlerType>,
    pub persistent_config: bool,
    pub core_db_path: String,
    pub server_key_path: String,
}

// todo: check that this lifetime is valid
impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            active_handlers: vec!(HandlerType::All),
            persistent_config: true,
            core_db_path: "./run/core.sqlite".to_owned(),
            server_key_path: "./run/private.pem".to_owned()
        }
    }
}

pub fn init() -> (CoreConfig, PKey<Private>) {

    let config: CoreConfig = match confy::load("quiver", None) {
        Ok(config) => {
            println!("Config Loaded...");
            config
        }
        Err(error) => {
            // Any errors that aren't a bad toml read will wind up being handled by the call
            // to confy::store. This is why individual confy::ConfyErrors aren't handled seperately. 
            println!("{error}");
            let mut config: CoreConfig = Default::default();
            match confy::store("quiver", None, &config) {
                Ok(_val) => println!("Default configuration generated and saved"),
                Err(err) => {
                    println!("!!Unable to store config due to: {err}\n
                                Continuing with default config 
                                without persistent config support!!");
                    config.persistent_config = false;
                }
            };
            config
        }
    };

    let server_key: PKey<Private> = match File::open(&config.server_key_path) {
        Err(e) => panic!("{}: {}", e, &config.server_key_path),
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut buff: Vec<u8> = Vec::new();
            match reader.read_to_end(&mut buff) {
                Err(_e) => unimplemented!(),
                Ok(_len) => {
                    match PKey::private_key_from_pem(&buff) {
                        Ok(key) => key,
                        Err(_e) => unimplemented!()
                    }
                }
            }
        }
    };
    // todo match on error kinds. Also, once std::fs::try_exists()
    // is pulled into stable, add that.
    remove_file("/tmp/quiver.calendar.sock");
    remove_file("/tmp/quiver.vpn.sock");
    remove_file("/tmp/quiver.nfc.sock");

    (config, server_key)
}