extern crate confy;
use crate::core::config::CoreConfig;

pub fn init() {
    let config: CoreConfig = match confy::load(&"quiver", "Core") {
        Ok(config) => {
            println!("Config Loaded...");
            config
        }
        Err(e) => {
            match e {
                _ => {
                    println!("{e}");
                    let config = Default::default();
                    match confy::store("quiver", None, &config) {
                        Ok(_v) => {
                            println!("Default configuration generated and saved...")
                        }
                        Err(e) => {
                            panic!("{e}");
                        }
                    }
                    config
                }
            }
        }

    };

    dbg!(config);    
}