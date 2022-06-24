use chrono::{Datelike, NaiveDate, Utc};
use clap::Parser;

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
}

fn format_date_args(date_string: &str) -> Result<NaiveDate, String> {
    let date = NaiveDate::parse_from_str(date_string, "%Y-%m-%d");
    match date {
        Ok(date) => Ok(date),
        Err(err) => Err(err.to_string()),
    }
}
