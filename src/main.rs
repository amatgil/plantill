pub mod parse;

use std::fs;

use toml::*;

use dialoguer::{theme::ColorfulTheme, Select, Input};

use crate::parse::parse_config;

// TODO:
// - [x] Parse toml, extract templates
// - [x] Ask for which to use
// - [x] Ask for project name
// - [ ] Verify that selected template folder exists
// - [ ] Replace corresponding keys with project name
//  - [ ] "PLANTILL_NAME" for the upper name
//      - [ ] In file names
//      - [ ] Inside files
//  - [ ] "plantill_name" for the lower name
//      - [ ] In file names
//      - [ ] Inside files
// - [ ] Try to copy it over to current location

fn main() {
    let tables = parse_config();
    let (selections, sources) = {
        let (mut sels, mut sources) = (vec![], vec![]);
        for (t, m) in tables {
            if let Value::String(v) = m["source"].clone() {
                sels.push(t);
                sources.push(v);
            } else { panic!("ERROR: {t} does not have its source folder in the config file"); }
        }
        (sels, sources)
    };
    //eprintln!("{:?}", selections);
    //eprintln!("{:?}", sources);

    let selection_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Template: ")
        .items(&selections[..])
        .interact()
        .unwrap();
    let source_path = format!("~/.config/plantill/{}", sources[selection_idx]);

    println!("Selected: '{}'. Will attempt to transfer over '{}'", selections[selection_idx], source_path);

    let project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .interact_text()
        .unwrap();

    println!("Continuing with project name: '{project_name}'");

    fs::copy(source_path, project_name).unwrap();
}

