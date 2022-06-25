use clap::Parser;
use std::fs::create_dir;
use std::path::PathBuf;
use std::process::Command;

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
    // Parse the command line arguments
    let args = Cli::parse();

    // Initialize git repository
    create_dir(&args.path).expect("failed to create a directory");
    Command::new("git")
        .args(&["init", "--quiet"])
        .current_dir(&args.path)
        .status()
        .expect("failed to initialize git repository");

    create_commit("fake commit", "Wed Feb 16 14:00 2011 +0100", &args.path);
}
