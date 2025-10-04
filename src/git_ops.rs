use std::{env, fs};
use std::fs::ReadDir;
use crate::dir_stack_guard::DirStackGuard;

pub fn fetch_all_repos(student_repos: ReadDir) {
    for repo_path in student_repos {
        let path = repo_path.unwrap().path().display().to_string();
        fetch(&path);
    }
}

fn fetch(dir: &String) {
    if fs::exists(dir).is_err() {
        println!("Skipping {dir} - no git-repository found!");
        return;
    }

    let _guard = DirStackGuard::push_dir(dir).unwrap();

    std::process::Command::new("git")
        .arg("fetch")
        .arg("origin")
        .spawn()
        .unwrap();
}