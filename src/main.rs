use std::io::{self, Write};

struct Config<'a> {
    language: &'a str,
    where_to_generate_addon: &'a str,
}

struct AddonTemplate<'a> {
    addon_name: &'a str,
    author_name: &'a str,
}

fn navigate() {
    println!("'Adgemin' マインクラフト アドオン テンプレート生成ソフト");
    println!("Created by eleven-junichi2");

    print!("What is your addon name?>");
    io::stdout().flush().unwrap();
    let mut addon_name = String::new();
    io::stdin().read_line(&mut addon_name).unwrap();
    let addon_name = addon_name.trim();

    print!("What is your name as author of your addon?>");
    io::stdout().flush().unwrap();
    let mut author_name = String::new();
    io::stdin().read_line(&mut author_name).unwrap();
    let author_name = author_name.trim();

    let new_addon = AddonTemplate {
        addon_name,
        author_name,
    };

    println!("Where do you want to generate this addon template?");
    println!("If you enter nothing, the location will be:");
    print!("{}>", "addon_template_path");
    io::stdout().flush().unwrap();
    let mut where_to_generate = String::new();
    io::stdin().read_line(&mut where_to_generate).unwrap();

    println!("---");
    println!("Addon's name: {}", new_addon.addon_name);
    println!("Author: {}", new_addon.author_name);
    println!("---");
}

fn main() {
    navigate();
}
