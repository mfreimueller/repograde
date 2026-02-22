use std::path::{Path, PathBuf};
use crate::config::Config;
use crate::git_ops::log;

pub struct RepoStats {
    pub(crate) project_name: String,
    pub(crate) range: String,
    pub(crate) lines_added: i32,
    pub(crate) lines_removed: i32,
    pub(crate) success: bool,
}

pub fn grade_student_repos(student_repos: &Vec<PathBuf>, from_date: &String, to_date: &String, config: &Config) -> Vec<RepoStats> {
    let mut repo_stats: Vec<RepoStats> = Vec::new();

    for repo_path in student_repos {
        let path = repo_path.display().to_string();

        let project_name = extract_project_name(&path, &config.prefix);

        let stats = analyze_repo(&path, from_date, to_date, project_name, config.minimum_commit_size);
        repo_stats.push(stats);
    }

    repo_stats
}

fn extract_project_name(path: &String, prefix: &String) -> String{
    Path::new(path.as_str())
        .file_name()
        .unwrap()
        .to_string_lossy()[prefix.len()..].to_string()
        .to_string()
}

fn analyze_repo(path: &String, from_date: &String, to_date: &String, project_name: String, minimum_commit_size: i32) -> RepoStats {
    let log_result = log(path, from_date, to_date);
    let range = format!("{from_date} - {to_date}");

    if log_result.is_err() {
        eprintln!("❌ ERROR: {project_name} ({log_result:?})");
        return RepoStats {
            project_name,
            range,
            lines_added: 0,
            lines_removed: 0,
            success: false,
        };
    }

    let mut added = 0;
    let mut removed = 0;

    let logs = log_result.unwrap();
    logs.split("\n").for_each(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        if parts.len() > 2 {
            added += parts[0].parse::<i32>().unwrap_or(0);
            removed -= parts[1].parse::<i32>().unwrap_or(0);
        }
    });

    let total_change_count = added - removed;
    let passes_minimum = total_change_count >= minimum_commit_size;

    if passes_minimum {
        println!("✅ PASS: {project_name} ({total_change_count} >= {minimum_commit_size})")
    } else {
        println!("❌ FAIL: {project_name} ({total_change_count} < {minimum_commit_size})")
    }

    RepoStats {
        project_name,
        range,
        lines_added: added,
        lines_removed: removed,
        success: passes_minimum,
    }
}