use std::fs::ReadDir;
use std::path::{Path, PathBuf};
use crate::git_ops::log;

struct RepoStats {
    lines_added: i32,
    lines_removed: i32,
    success: bool,
}

impl RepoStats {}

pub fn grade_student_repos(student_repos: &Vec<PathBuf>) {
    for repo_path in student_repos {
        let path = repo_path.display().to_string();

        // currently we have the prefix "team-project-" hardcoded.
        // TODO: consider making the prefix configurable
        let project_name = Path::new(path.as_str())
            .file_name()
            .unwrap()
            .to_string_lossy()[13..]
            .to_string();

        let stats = analyze_repo(path, project_name);
    }
}

fn analyze_repo(path: String, project_name: String) -> RepoStats {
    let log_result = log(path);

    if log_result.is_err() {
        eprintln!("❌ FAIL: {project_name}");
        return RepoStats {
            lines_added: 0,
            lines_removed: 0,
            success: false,
        };
    }

    let mut added = 0;
    let mut removed = 0;

    let logs = log_result.unwrap();
    logs.split('\n').for_each(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        added += parts[0].parse::<i32>().unwrap();
        removed -= parts[1].parse::<i32>().unwrap();
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
        lines_added: added,
        lines_removed: removed,
        success: passes_minimum,
    }
}