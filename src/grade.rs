use std::path::{Path, PathBuf};
use crate::git_ops::log;

pub struct RepoStats {
    pub(crate) project_name: String,
    pub(crate) date: String,
    pub(crate) lines_added: i32,
    pub(crate) lines_removed: i32,
    pub(crate) success: bool,
}

impl RepoStats {}

pub fn grade_student_repos(student_repos: &Vec<PathBuf>, date: &String) -> Vec<RepoStats> {
    let mut repo_stats: Vec<RepoStats> = Vec::new();

    for repo_path in student_repos {
        let path = repo_path.display().to_string();

        // currently we have the prefix "team-project-" hardcoded.
        // TODO: consider making the prefix configurable
        let project_name = extract_project_name(&path);

        let stats = analyze_repo(&path, date, project_name);
        repo_stats.push(stats);
    }

    repo_stats
}

fn extract_project_name(path: &String) -> String{
    Path::new(path.as_str())
        .file_name()
        .unwrap()
        .to_string_lossy()[13..]
        .to_string()
}

fn analyze_repo(path: &String, date: &String, project_name: String) -> RepoStats {
    let date = String::from(date);
    let log_result = log(path, &date);

    if log_result.is_err() {
        eprintln!("❌ FAIL: {project_name}");
        return RepoStats {
            project_name,
            date,
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
            added += parts[0].parse::<i32>().unwrap();
            removed -= parts[1].parse::<i32>().unwrap();
        }
    });

    let total_change_count = added - removed;
    // TODO: make 50 (i.e. a 'substantial commit size' configurable)
    let passes_minimum = total_change_count >= 50;

    if passes_minimum {
        println!("✅ PASS: {project_name}")
    } else {
        println!("❌ FAIL: {project_name}")
    }

    RepoStats {
        project_name,
        date,
        lines_added: added,
        lines_removed: removed,
        success: passes_minimum,
    }
}