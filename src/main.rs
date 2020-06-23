use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Write};
use std::path;

struct Config<'a> {
    language: &'a str,
    where_to_generate_addon: &'a str,
}

struct AddonTemplate<'a> {
    addon_name: &'a str,
    author_name: &'a str,
}

fn generate_addon() {}

fn generate_behavior_pack() {}

fn generate_resource_pack() {}

fn behavior_pack_manifest() {}

fn resource_pack_manifest() {}

fn load_translation(language: &str) -> HashMap<String, String> {
    // let current_dir = &env::current_dir().unwrap();
    let current_dir = path::Path::new(&env::current_dir().unwrap())
        .join("translation")
        .join(format!("{}.json", language));
    println!("debubbbgggg {:?}", current_dir);
    let config_file = File::open(current_dir).unwrap();
    let reader = BufReader::new(config_file);
    let tl: HashMap<String, String> = serde_json::from_reader(reader).unwrap();
    return tl;
    // return HashMap::new();
}

fn navigate() {
    let tl = load_translation("jp");
    println!("{}", tl["title"]);
    println!("{}", tl["credit"]);

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

    let new_addon = AddonTemplate {
        addon_name,
        author_name,
    };

    println!("{}", tl["input_location"]);
    println!("{}", tl["if_you_enter_nothing"]);
    print!("{}>", "addon_template_path");
    io::stdout().flush().unwrap();
    let mut where_to_generate = String::new();
    io::stdin().read_line(&mut where_to_generate).unwrap();

    println!("---");
    println!("{} {}", tl["result_addon_name"], new_addon.addon_name);
    println!("{} {}", tl["result_author_name"], new_addon.author_name);
    println!("---");
}

fn main() {
    navigate();
}
