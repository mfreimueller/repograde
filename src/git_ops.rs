use std::io::{Error, ErrorKind};
use std::path::{PathBuf};
use crate::dir_stack_guard::DirStackGuard;
use futures::future::join_all;
use tokio::process::Command;

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

pub async fn fetch_all_repos(student_repos: &Vec<PathBuf>) {
    let fetches = student_repos.into_iter().map(|repo_path| async move {
        println!("Fetching {:?}", repo_path);
        let status = Command::new("git")
            .arg("fetch")
            .arg("origin")
            .current_dir(repo_path)
            .status()
            .await?;

        if status.success() {
            Ok(())
        } else {
            println!("❌ Fetch failed for {}", repo_path.display());
            Err(anyhow::anyhow!("Fetch failed"))
        }
    });

    join_all(fetches).await;
}
