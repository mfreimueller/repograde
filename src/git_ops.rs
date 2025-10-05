use std::{fs};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use crate::dir_stack_guard::DirStackGuard;

pub fn log(path: &String, date: &String) -> Result<String, Error> {
    let _guard = DirStackGuard::push_dir(path)?;

    let since = format!("--since={}", date);
    let output = std::process::Command::new("git")
        .arg("log")
        .arg(since)
        .arg("--pretty=tformat:")
        .arg("--numstat")
        .arg("--all")
        .output()?;

    if !output.status.success() {
        eprintln!("git log failed {:?}", output.status);
        return Err(Error::new(
            ErrorKind::Other,
            format!("git log failed {:?}", output.status),
        ))
    }

    let out = String::from_utf8(output.stdout).unwrap();
    Ok(out)
}

pub fn fetch_all_repos(student_repos: &Vec<PathBuf>) {
    for repo_path in student_repos {
        let path = repo_path.display().to_string();
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