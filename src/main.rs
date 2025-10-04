/*
 * repograde - CLI tool to automate the fetching and grading of GitHub classroom
 * repositories.
 *
 * USAGE: cargo run -- [DIRECTORY]
 */
mod git_ops;
mod cli_ops;

use std::io::Write;
use std::{env, fs};
use crate::git_ops::update_repos;

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

    update_repos(root_dir);
}
