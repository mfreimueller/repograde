use std::{env, fs};
use crate::cli_ops::{push_dir, pop_dir};

pub fn update_repos(root_dir: &String) {
    let paths = fs::read_dir(root_dir).unwrap();
    for path in paths {
        let path = path.unwrap().path().display().to_string();
        fetch(&path);
    }
}

fn fetch(dir: &String) {
    if fs::exists(dir).is_err() {
        println!("Skipping {dir} - no git-repository found!");
        return;
    }

    let current_dir = env::current_dir().unwrap();
    env::set_current_dir(dir).unwrap();

    std::process::Command::new("git")
        .arg("fetch")
        .arg("origin")
        .spawn()
        .unwrap();

    env::set_current_dir(current_dir).unwrap();
}