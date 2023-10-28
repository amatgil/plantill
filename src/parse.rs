use std::{fs::{File, self}, collections::HashMap, borrow::Cow};


use toml::Value;

use crate::CONFIG_ROOT;

pub fn parse_config() -> Vec<(String, toml::map::Map<String, Value>)> {
    let config_path = format!("{}/config.toml", CONFIG_ROOT);
    let toml_content = fs::read_to_string(&*shellexpand::tilde(&config_path))
        .expect("ERROR: Could not find config file.");

    let toml_value: Result<Value, _> = toml_content.parse();

    let mut out = vec![];
    match toml_value {
        Ok(value) => {
            if let Value::Table(table) = value {
                for (key, subvalue) in table.iter() {
                    if let Value::Table(t) = subvalue {
                        println!("Detected table: {}", key);
                        out.push((key.clone(), t.clone()));
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing TOML content: {}", e);
        }
    }
    out
}
