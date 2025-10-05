use std::{any, fs};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use crate::grade::RepoStats;

pub fn get_student_repo_paths(root_dir: &String) -> Vec<PathBuf> {
    fs::read_dir(root_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect()
}

pub fn write_repo_stats_to_csv_file(repo_stats: Vec<RepoStats>) -> anyhow::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs.csv")?;

    for stat in repo_stats {
        write_stat(&mut file, stat)?;
    }

    Ok(())
}

fn write_stat(file: &mut File, stat: RepoStats) -> std::io::Result<()> {
    let date = stat.date;
    let project_name = stat.project_name;
    let added = stat.lines_added;
    let removed = stat.lines_removed;
    let status = if stat.success { "PASS" } else { "FAIL" };

    writeln!(file, "{date};{project_name};{added};{removed};{status}")
}