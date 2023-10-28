pub mod parse;

use std::{fs::{self, File}, io::Write};
use copy_dir::copy_dir;

use toml::*;

use dialoguer::{theme::ColorfulTheme, Select, Input};

use crate::parse::parse_config;

// TODO:
// - [x] Parse toml, extract templates
// - [x] Ask for which to use
// - [x] Ask for project name
// - [x] Verify that selected template folder exists (implicit)
// - [x] Try to copy it over to current location
// - [ ] Replace corresponding keys with project name
//  - [ ] "PLANTILL_NAME" for the upper name
//      - [ ] In file names
//      - [ ] Inside files
//  - [ ] "plantill_name" for the lower name
//      - [ ] In file names
//      - [ ] Inside files

const CONFIG_ROOT: &str = "~/.config/plantill/";

fn main() {
    let tables = parse_config();
    let (selections, sources, should_replace) = {
        let (mut sels, mut sources, mut repl) = (vec![], vec![], vec![]);
        for (t, m) in tables {
            if let Value::String(v) = m["source"].clone() {
                sels.push(t.clone());
                sources.push(v);
                if let Value::Boolean(v) = m["should_replace_name"].clone() {
                    repl.push(v);
                } else { 
                    panic!("[CONFIG ERROR]: {} does not specify if it should have its name changed folder in the config file", &t); 
                }
            } else { panic!("[CONIG ERROR]: {t} does not have its source folder in the config file"); }
        }
        (sels, sources, repl)
    };

    let selection_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Template: ")
        .items(&selections[..])
        .interact() .unwrap();

    let source_path = format!("{}{}", shellexpand::tilde(CONFIG_ROOT), sources[selection_idx]);

    println!("Selected: '{}'. Will attempt to transfer over '{}'", selections[selection_idx], source_path);

    let project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .interact_text()
        .unwrap();

    println!("Continuing with project name: '{project_name}'");

    copy_dir(source_path, &project_name).unwrap();

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let mut file_contents = fs::read_to_string(&path).unwrap();
        file_contents = file_contents.replace("PLANTILL_NAME", &project_name.to_uppercase());
        let mut file = File::open(path).unwrap(); 
        file.write(file_contents.as_bytes()).unwrap();
    }
}

