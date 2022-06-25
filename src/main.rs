use chrono::{Datelike, NaiveDate, Utc};
use clap::Parser;
use std::fs::create_dir;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Commit only on workdays
    #[clap(short, long, value_parser, default_value_t = false)]
    workdays: bool,

    /// Maximum number of commits per day
    #[clap(short = 'c', long = "commits", value_parser, default_value_t = 3)]
    max_commits: u32,

    /// Start date of the range to commit, if not specified current day is used
    /// Format : YYYY-MM-DD
    #[clap(long, value_parser = format_date_args, default_value_t = NaiveDate::from_ymd(Utc::today().year(), 1, 1))]
    start_date: NaiveDate,

    /// End date of the range to commit, if not specified current day is used
    /// Format : YYYY-MM-DD
    #[clap(long, value_parser = format_date_args, default_value_t = Utc::today().naive_utc())]
    end_date: NaiveDate,

    #[clap(short, long, parse(from_os_str), default_value = "./fake-repo")]
    path: PathBuf,
}

fn format_date_args(date_string: &str) -> Result<NaiveDate, String> {
    let date = NaiveDate::parse_from_str(date_string, "%Y-%m-%d");
    match date {
        Ok(date) => Ok(date),
        Err(err) => Err(err.to_string()),
    }
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
        .expect("failed to create a commit");
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

    // For each day in the range, commit a random number of times
    let mut current_date = args.start_date;
    while current_date <= args.end_date {
        create_commit(
            "fake commit",
            &current_date.format("%Y-%m-%d").to_string(),
            &args.path,
        );

        current_date = current_date + chrono::Duration::days(1);
    }
}
