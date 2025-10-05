/*
 * repograde - CLI tool to automate the fetching and grading of GitHub classroom
 * repositories.
 *
 * USAGE: cargo run -- [DIRECTORY]
 */
mod git_ops;
mod grade;
mod dir_stack_guard;

use std::io::Write;
use std::{env, fs};
use std::path::PathBuf;
use crate::git_ops::fetch_all_repos;
use crate::grade::grade_student_repos;

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

    // TODO: write args utility
    let date: String;
    if args.len() > 2 {
        if is_valid_date_string(&args[2]) {
            date = args[2].to_string();
        } else {
            date = yesterday_string();
        }
    } else {
        date = yesterday_string();
    }

    fetch_all_repos(&student_repos);
    grade_student_repos(&student_repos);
    let repo_stats = grade_student_repos(&student_repos, &date);
}
