/// A personal project to use templates so that starting projects in languages that don't have a cargo new equivalent doesn't take so long (e.g. LaTeX or Common Lisp).
pub mod parse;
use crate::parse::parse_config;
use copy_dir::copy_dir;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::{
    fs::{self, remove_file, File},
    io::Write,
};
use toml::Value;

/// The directory where the configuration and templates must be saved. As of now, it is immutable.
pub const CONFIG_ROOT: &str = "~/.config/plantill/";

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
            } else {
                panic!("[CONIG ERROR]: {t} does not have its source folder in the config file");
            }
        }
        (sels, sources, repl)
    };

    let selection_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Template: ")
        .items(&selections[..])
        .interact()
        .unwrap();

    let source_path = format!(
        "{}{}",
        shellexpand::tilde(CONFIG_ROOT),
        sources[selection_idx]
    );

    println!(
        "Selected: '{}'. Will attempt to transfer over '{}'",
        selections[selection_idx], source_path
    );

    let project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .interact_text()
        .unwrap();

    println!("Continuing with project name: '{project_name}'");

    copy_dir(source_path, &project_name).unwrap();

    let paths = fs::read_dir(&project_name).unwrap();

    if !should_replace[selection_idx] {
        std::process::exit(0); // Don't need to replace, we're done
    }

    for dir_entry in paths {
        if dir_entry.as_ref().unwrap().path().is_dir() { continue; }
        let path = dir_entry.unwrap().path();

        // --------- Contents ----------
        let mut file_contents = fs::read_to_string(&path).unwrap();

        file_contents = file_contents.replace("PLANTILLNAME", &project_name.to_uppercase());
        file_contents = file_contents.replace("plantillname", &project_name);
        let mut file = File::create(&path).unwrap();

        file.write_all(file_contents.as_bytes()).unwrap();

        // --------- Filenames ----------
        let new_path = path
            .clone()
            .to_str()
            .to_owned()
            .unwrap()
            .replace("plantillname", &project_name);

        remove_file(path).unwrap();
        let mut new_file = File::create(&new_path).unwrap();
        new_file.write_all(file_contents.as_bytes()).unwrap();
    }
}
