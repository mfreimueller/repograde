/*
 * repograde - CLI tool to automate the fetching and grading of GitHub classroom
 * repositories.
 *
 * USAGE: cargo run -- [DIRECTORY] [DATE (optional)]
 */
mod git_ops;
mod grade;
mod dir_stack_guard;
mod file_ops;
mod date_util;

use std::io::Write;
use std::{env, fs};
use crate::date_util::{is_valid_date_string, yesterday_string};
use crate::file_ops::{get_student_repo_paths, write_repo_stats_to_csv_file};
use crate::git_ops::fetch_all_repos;
use crate::grade::grade_student_repos;

fn main() -> std::io::Result<()> {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        writeln!(&mut std::io::stderr(), "Usage: cargo run -- [DIRECTORY] [DATE (optional)]")?;
        std::process::exit(exitcode::USAGE);
    }

    let root_dir = &args[1];
    if fs::exists(root_dir).is_err() {
        writeln!(&mut std::io::stderr(), "{root_dir} doesn't exist or the required permissions are not set.")?;
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

    let config = read_config("config.toml")?;

    let student_repos = get_student_repo_paths(root_dir);
    fetch_all_repos(&student_repos);
    let repo_stats = grade_student_repos(&student_repos, &date);
    write_repo_stats_to_csv_file(repo_stats)
}
