use std::path::PathBuf;

use clap::Parser;
use git2::Repository;

#[derive(Parser)]
struct Cli {
    /// Path of the initialized repository
    #[clap(short, long, parse(from_os_str), default_value = "./fake-repo")]
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let repo = match Repository::init(args.path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
}
