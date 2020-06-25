use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Write};
use std::path;

struct AddonTemplate<'a> {
    addon_name: &'a str,
    author_name: &'a str,
    where_to_make: &'a path::Path,
    using_template_path: &'a path::Path,
}

impl<'a> AddonTemplate<'a> {
    fn generate_addon(self) {}

    fn generate_behavior_pack(self) {}

    fn generate_resource_pack(self) {}
    fn behavior_pack_manifest(self) {}
    fn resource_pack_manifest(self) {}
}

fn load_config(file_dir: &path::Path) -> HashMap<String, String> {
    let file_path = file_dir.join("config.json");
    let config_file = File::open(file_path).unwrap();
    let reader = BufReader::new(config_file);
    let config: HashMap<String, String> = serde_json::from_reader(reader).unwrap();
    config
}

fn load_translation(language: &str, file_dir: &path::Path) -> HashMap<String, String> {
    let file_path = file_dir.join(format!("{}.json", language));
    let translation_file = File::open(file_path).unwrap();
    let reader = BufReader::new(translation_file);
    let tl: HashMap<String, String> = serde_json::from_reader(reader).unwrap();
    tl
}

fn navigate() {
    let cur_exe_dir = &env::current_exe().unwrap();
    let config = load_config(&path::Path::new(&cur_exe_dir).parent().unwrap());
    println!("cur_exe: {:?}", env::current_exe().unwrap());
    let translation_file_dir = path::Path::new(&env::current_exe().unwrap())
        .parent()
        .unwrap()
        .join("translation");
    let tl = load_translation(&config["language"], &translation_file_dir);
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
    let where_to_make = path::Path::new(&cur_exe_dir).parent().unwrap();
    let using_template_path = path::Path::new(&cur_exe_dir).parent().unwrap();
    let new_addon = AddonTemplate {
        addon_name,
        author_name,
        where_to_make,
        using_template_path,
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
