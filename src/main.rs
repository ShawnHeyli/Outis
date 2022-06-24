use clap::Parser;
use std::env;

#[derive(Parser)]
struct Cli {
    /// Path of the initialized repository
    #[clap(short, long)]
    path: String,
}

fn main() {}
