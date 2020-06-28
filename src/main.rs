use copy_dir;
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::BufReader;
use std::io::{self, Write};
use std::path;
use std::ffi::OsString;
use uuid::Uuid;

struct AddonTemplate<'a> {
    addon_name: &'a str,
    author_name: &'a str,
    where_to_make: &'a path::Path,
    using_template_dir: &'a path::Path,
}

impl<'a> AddonTemplate<'a> {
    fn generate_addon<'b>(&'b self) {
        println!("ge1: {:?}", self.using_template_dir);
        println!("ge2: {:?}", self.where_to_make.join(self.addon_name));
        println!(
            "readdir: {:?}",
            fs::read_dir(self.using_template_dir.join("templateBP")).unwrap()
        );
        fs::create_dir(self.where_to_make.join(self.addon_name));
        self.generate_behavior_pack();
        self.generate_resource_pack();
    }

    fn generate_behavior_pack<'b>(&'b self) {
        copy_dir::copy_dir(
            self.using_template_dir.join("templateBP"),
            self.where_to_make
                .join(self.addon_name)
                .join(format!("{}BP", self.addon_name)),
        );
        let manifest_json = self.behavior_pack_manifest();
        let manifest_writer = fs::File::create(
            self.where_to_make
                .join(self.addon_name)
                .join(format!("{}BP", self.addon_name))
                .join("manifest.json"),
        )
        .unwrap();
        serde_json::to_writer_pretty(manifest_writer, &manifest_json);
    }
    fn generate_resource_pack<'b>(&'b self) {
        copy_dir::copy_dir(
            self.using_template_dir.join("templateRP"),
            self.where_to_make
                .join(self.addon_name)
                .join(format!("{}RP", self.addon_name)),
        );
        let manifest_json = self.resource_pack_manifest();
        let manifest_writer = fs::File::create(
            self.where_to_make
                .join(self.addon_name)
                .join(format!("{}RP", self.addon_name))
                .join("manifest.json"),
        )
        .unwrap();
        serde_json::to_writer_pretty(manifest_writer, &manifest_json);
    }
    fn resource_pack_manifest<'b>(&'b self) -> serde_json::Value {
        let rp_uuid = Uuid::new_v4();
        let rp_modules_uuid = Uuid::new_v4();
        let rp_manifest = serde_json::json!(
            {
                "format_version": 2,
                "header": {
                    "description": format!("Created by {}", self.author_name),
                    "name": format!("{} Resource Pack", self.addon_name),
                    "uuid": &rp_uuid,
                    "version": [0, 0, 1],
                    "min_engine_version": [ 1, 14, 0 ]
                },
                "modules": [
                    {
                        "description": format!("{} Resource Pack", self.addon_name),
                        "type": "resources",
                        "uuid": &rp_modules_uuid,
                        "version": [0, 0, 1]
                    }
                ]
            }
        );
        return rp_manifest;
    }
    fn behavior_pack_manifest<'b>(&'b self) -> serde_json::Value {
        let bp_uuid = Uuid::new_v4();
        let bp_modules_uuid = Uuid::new_v4();
        let bp_manifest = serde_json::json!(
            {
                "format_version": 2,
                "header": {
                    "description": format!("Created by {}", self.author_name),
                    "name": format!("{} Behavior Pack", self.addon_name),
                    "uuid": &bp_uuid,
                    "version": [ 0, 0, 1 ],
                    "min_engine_version": [ 1, 14, 0 ]
                },
                "modules": [
                    {
                        "description": format!("{} Behavior Pack", self.addon_name),
                        "type": "data",
                        "uuid": &bp_modules_uuid,
                        "version": [0, 0, 1]
                    }
                ]
            }
        );
        return bp_manifest;
    }
}

fn load_config(file_dir: &path::Path) -> HashMap<String, String> {
    let file_path = file_dir.join("config.json");
    let config_file = fs::File::open(file_path).unwrap();
    let reader = BufReader::new(config_file);
    let config: HashMap<String, String> = serde_json::from_reader(reader).unwrap();
    config
}

fn load_translation(language: &str, file_dir: &path::Path) -> HashMap<String, String> {
    let file_path = file_dir.join(format!("{}.json", language));
    let translation_file = fs::File::open(file_path).unwrap();
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
    println!("test1 {:?}", &where_to_make);
    let new_addon = AddonTemplate {
        addon_name,
        author_name,
        where_to_make,
        using_template_dir,
    };
    new_addon.generate_addon();
    println!("---");
    println!("{} {}", tl["result_addon_name"], &new_addon.addon_name);
    println!("{} {}", tl["result_author_name"], &new_addon.author_name);
    println!("---");
}

fn main() {
    navigate();
}
