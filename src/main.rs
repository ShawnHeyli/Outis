use clap::Parser;
use git2::Repository;

#[derive(Parser)]
struct Cli {
    /// Path of the initialized repository
    #[clap(short, long)]
    path: String,
}

fn main() {
    let args = Cli::parse();
    let repo = match Repository::init(args.path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
}
