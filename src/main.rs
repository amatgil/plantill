/// A personal project to use templates so that starting projects in languages that don't have a cargo new equivalent doesn't take so long (e.g. LaTeX or Common Lisp).
pub mod parse;
use crate::parse::parse_config;
use std::{
    env::args,
    fs::{self, remove_file, File},
    io::Write,
    path::Path,
    process::exit,
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

    let mut args = args();
    _ = args.next(); // binary name

    let Some(arg) = args.next() else {
        eprintln!("You forgot the argument (use -options to list them)");
        exit(1)
    };
    if ["-help", "--help"].contains(&&*arg) {
        println!("'-help' for help, '-options' for options");
        exit(0);
    } else if ["-options", "--options"].contains(&&*arg) {
        println!("Options are:");
        for s in selections {
            match s.strip_prefix("copet-") {
                Some(langname) => println!("\t{s}\t({langname})"),
                None => println!("\t{s}"),
            }
        }
        exit(0);
    }
    let Some(project_name) = args.next() else {
        eprintln!("You forgot the name of the project (second argument, the one after the name)");
        exit(1)
    };

    // We got option
    let Some(selection_idx) = selections
        .iter()
        .position(|s| s == &arg || s == &format!("copet-{arg}"))
    else {
        println!("'{arg}' is not recognized as an option (use -options to list them)");
        exit(2);
    };

    let source_path = format!(
        "{}{}",
        shellexpand::tilde(CONFIG_ROOT),
        sources[selection_idx]
    );

    println!(
        "Selected: '{}'. Will attempt to transfer over '{}'",
        selections[selection_idx], source_path
    );

    println!("Continuing with project name: '{project_name}'");

    copy_dir(&source_path, &project_name).unwrap();
    println!("Copying succeeded");

    let paths = fs::read_dir(&project_name).unwrap();

    if !should_replace[selection_idx] {
        std::process::exit(0); // Don't need to replace, we're done
    }

    for dir_entry in paths {
        if dir_entry.as_ref().unwrap().path().is_dir() {
            continue;
        }
        println!("Changing names in: {dir_entry:?}");
        let path = dir_entry.unwrap().path();

        // --------- Contents ----------
        let mut file_contents = fs::read_to_string(&path).unwrap();

        file_contents = file_contents.replace("PLANTILLNAME", &project_name.to_uppercase());
        file_contents = file_contents.replace("plantillname", &project_name);
        remove_file(path.clone()).unwrap();
        let mut file = File::create(&path).unwrap();
        file.write_all(file_contents.as_bytes()).unwrap(); // This line might be
                                                           // redundant, not sure

        // --------- Filenames ----------
        let new_path = path
            .clone()
            .to_str()
            .to_owned()
            .unwrap()
            .replace("plantillname", &project_name);

        if path.to_str().expect("Path wasn't valid unicode") != new_path {
            remove_file(path).unwrap();
            let mut new_file = File::create(&new_path).unwrap();
            new_file.write_all(file_contents.as_bytes()).unwrap();
        }
    }
}

fn copy_dir(
    from_path: impl AsRef<Path>,
    to_path: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir(&to_path)?;
    for entry in fs::read_dir(from_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir(entry.path(), to_path.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), to_path.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
