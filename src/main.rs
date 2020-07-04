use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::{self, Write};
use std::io::{stdin, stdout, Read};
use std::path;

mod addon_generater;

fn pause() {
    let mut stdout = stdout();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn load_config(file_dir: &path::Path) -> Result<HashMap<String, String>, io::Error> {
    let file_path = file_dir.join("config.json");
    let config_file = fs::File::open(file_path)?;
    let reader = BufReader::new(config_file);
    let config: HashMap<String, String> = serde_json::from_reader(reader).unwrap();
    Ok(config)
}

fn load_translation(
    language: &str,
    file_dir: &path::Path,
) -> Result<HashMap<String, String>, io::Error> {
    let file_path = file_dir.join(format!("{}.json", language));
    let translation_file = fs::File::open(file_path)?;
    let reader = BufReader::new(translation_file);
    let tl: HashMap<String, String> = serde_json::from_reader(reader).unwrap();
    Ok(tl)
}

fn navigate() {
    let cur_exe_dir = &env::current_exe().unwrap();
    let config = load_config(&path::Path::new(&cur_exe_dir).parent().unwrap());
    let config = match config {
        Ok(config_data) => config_data,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            println!(
                "Error: The config.json was not found. \n
                 Please put it at same directry as the application."
            );
            pause();
            panic!("{}", error)
        }
        Err(error) => panic!("{}", error),
    };
    let translation_file_dir = path::Path::new(&env::current_exe().unwrap())
        .parent()
        .unwrap()
        .join("translation");
    let tl = load_translation(&config["language"], &translation_file_dir);
    let tl = match tl {
        Ok(translation) => translation,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            println!("Error: the translation file was not found.");
            pause();
            panic!("{}", error)
        }
        Err(error) => panic!("{}", error),
    };
    println!("{}", tl["title"]);
    println!("{}", tl["credit"]);
    loop {
        //input addon information
        print!("{}", tl["input_addon_name"]);
        io::stdout().flush().unwrap();
        let mut addon_name = String::new();
        io::stdin().read_line(&mut addon_name).unwrap();
        let addon_name = addon_name.trim();
        print!("{}", tl["input_author_name"]);
        io::stdout().flush().unwrap();
        let mut author_name = String::new();
        io::stdin().read_line(&mut author_name).unwrap();
        let author_name = author_name.trim();
        path::Path::new(&config["generating_location"]);
        let using_template_dir = &path::Path::new(&cur_exe_dir)
            .parent()
            .unwrap()
            .join("addon_template");
        println!("{}", tl["input_location"]);
        println!("{}", tl["if_you_enter_nothing"]);
        print!("{}>", &config["generating_location"]);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_where_to_make = input.trim();
        let where_to_make = if input_where_to_make.is_empty() {
            path::Path::new(&config["generating_location"])
        } else {
            path::Path::new(&input_where_to_make)
        };
        let new_addon = addon_generater::AddonTemplate {
            addon_name,
            author_name,
            where_to_make,
            using_template_dir,
        };
        match new_addon.generate_addon() {
            Ok(_) => (),
            Err(error) => {
                match error.kind() {
                    ErrorKind::NotFound => {
                        println!("{} {}", tl["path_not_exist_err"], where_to_make.display());
                        continue;
                    },
                    ErrorKind::AlreadyExists => {
                        println!("{} {}", tl["addon_is_already_exist_err"], addon_name);
                        continue;
                    },
                    _ => panic!("{}", error)
                }
            }
        };
        println!("---");
        println!("{} {}", tl["result_addon_name"], &new_addon.addon_name);
        println!("{} {}", tl["result_author_name"], &new_addon.author_name);
        println!("{} {}", tl["result_generated_in"], &new_addon.where_to_make.display());
        println!("---");
        pause();
        break
    }
}

fn main() -> Result<(), io::Error> {
    navigate();
    Ok(())
}
