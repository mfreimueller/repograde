/*
 * repograde - CLI tool to automate the fetching and grading of GitHub classroom
 * repositories.
 *
 * USAGE: cargo run -- [DIRECTORY]
 */
mod git_ops;
mod cli_ops;
mod dir_stack_guard;

use std::io::Write;
use std::{env, fs};
use std::path::PathBuf;
use crate::git_ops::fetch_all_repos;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        writeln!(&mut std::io::stderr(), "Usage: cargo run -- [DIRECTORY]").unwrap();
        std::process::exit(exitcode::USAGE);
    }

    let root_dir = &args[1];
    if fs::exists(root_dir).is_err() {
        writeln!(&mut std::io::stderr(), "{root_dir} doesn't exist or the required permissions are not set.").unwrap();
        std::process::exit(exitcode::IOERR);
    }

    let student_repos: Vec<PathBuf> = fs::read_dir(root_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();

    fetch_all_repos(student_repos);
    fetch_all_repos(&student_repos);
}
