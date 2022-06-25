use std::path::PathBuf;

use clap::Parser;
use git2::Repository;

#[derive(Parser)]
struct Cli {
    /// Path of the initialized repository
    #[clap(short, long, parse(from_os_str), default_value = "./fake-repo")]
    path: PathBuf,
}

#[cfg(target_family = "windows")]
fn create_commit(message: &str, date: &str, repo_path: &PathBuf) {
    Command::new("cmd")
        .args(&[
            "/C",
            "git",
            "commit",
            "--quiet",
            "--allow-empty",
            "-m",
            message,
            "--date",
            date,
        ])
        .current_dir(repo_path)
        .status()
        .expect("failed to create a commit");
}

#[cfg(target_family = "unix")]
fn create_commit(message: &str, date: &str, repo_path: &PathBuf) {
    Command::new("sh")
        .args(&[
            "-c",
            "git",
            "commit",
            "--quiet",
            "--allow-empty",
            "-m",
            message,
            "--date",
            date,
        ])
        .current_dir(repo_path)
        .status()
        .expect("failed to create a commit")
}

fn main() {
    let args = Cli::parse();
    let repo = match Repository::init(args.path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
}
