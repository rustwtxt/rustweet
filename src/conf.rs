//
// rustweet - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//
use serde::{Deserialize, Serialize};
use serde_yaml;

use std::fs;
use std::process;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub nick: String,
    pub path: String,
    pub url: String,
    pub follow: Vec<String>,
}

lazy_static! {
    pub static ref DATA: Arc<Data> = Arc::new(init());
    pub static ref FILE: String = {
        format!(
            "{}/.config/rustweet",
            std::env::var("HOME").unwrap_or_else(|_| ".".into())
        )
    };
}

pub fn init() -> Data {
    let file = format!("{}", *FILE);

    if !fs::metadata(&file).is_ok() {
        eprintln!();
        eprintln!("Configuration file missing: $HOME/.config/rustweet\nFor instructions, please see:\n\t$ rustweet --manual");
        eprintln!();
        process::exit(1);
    }

    let conf_as_str = match fs::read_to_string(&file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!();
            eprintln!(
                "Can't read configuration file: $HOME/.config/rustweet -- {:?}",
                err
            );
            eprintln!();
            process::exit(1);
        }
    };

    match serde_yaml::from_str::<Data>(&conf_as_str) {
        Ok(data) => return data,
        Err(err) => {
            eprintln!();
            eprintln!(
                "Improperly formatted configuration file: $HOME/.config/rustweet: {:?}",
                err
            );
            eprintln!();
            process::exit(1);
        }
    };
}
