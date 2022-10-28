use confy;
use crate::{
    core::{
    config::CoreConfig,
    },
};
use std::fs::remove_file;

pub fn init() -> CoreConfig {

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

    // todo match on error kinds. Also, once std::fs::try_exists()
    // is pulled into stable, add that.
    remove_file("/tmp/quiver.calendar.sock");
    remove_file("/tmp/quiver.vpn.sock");
    remove_file("/tmp/quiver.nfc.sock");

    config
}