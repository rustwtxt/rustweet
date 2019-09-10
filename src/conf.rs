use lazy_static::lazy_static;

use std::collections::HashMap;

lazy_static! {
    pub static ref DATA: HashMap<String, String> = HashMap::new();
}

pub fn init() {
    println!("TEST");
}
