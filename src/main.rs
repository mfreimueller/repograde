/*
 * repograde - CLI tool to automate the fetching and grading of GitHub classroom
 * repositories.
 */
mod git_ops;
mod grade;
mod dir_stack_guard;
mod file_ops;
mod date_util;
mod config;
mod cli_args;

use clap::Parser;
use crate::cli_args::Args;
use crate::config::read_config;
use crate::date_util::{is_valid_date_string, today_string, yesterday_string};
use crate::file_ops::{get_student_repo_paths, write_repo_stats_to_csv_file};
use crate::git_ops::fetch_all_repos;
use crate::grade::grade_student_repos;

fn main() -> anyhow::Result<()> {
    let mut args = Args::parse();
    if args.to_date.is_empty() || !is_valid_date_string(&args.to_date) {
        args.to_date = today_string();
    }

    if args.from_date.is_empty() || !is_valid_date_string(&args.from_date) {
        args.from_date = yesterday_string();
    }

    if std::fs::metadata(&args.config_file).is_err() {
        println!("Config file does not exist");
        std::process::exit(1);
    }

    let config = read_config(args.config_file)?;

    let student_repos = get_student_repo_paths();

    println!( "Fetching all repositories...");
    let _ = tokio::runtime::Runtime::new()?
        .block_on(fetch_all_repos(&student_repos));

    let repo_stats = grade_student_repos(&student_repos, &args.from_date, &args.to_date, &config);

    if !args.dry_run {
        write_repo_stats_to_csv_file(repo_stats, &config)
    } else {
        Ok(())
    }
}
