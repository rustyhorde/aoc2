use clap::{ArgAction::Count, Parser};
use getset::Getters;

const DEFAULT_YEAR: &str = "2020";

#[derive(Clone, Debug, Getters, Parser)]
#[command(author, version, about = "Run Advent of Code daily problems", long_about = None)]
#[getset(get = "pub(crate)")]
pub(crate) struct Args {
    /// Set logging verbosity.  More v's, more verbose.
    #[clap(
        short,
        long,
        action = Count,
        help = "Turn up logging verbosity (multiple will turn it up more)",
        conflicts_with = "quiet",
    )]
    verbose: u8,
    /// Set logging quietness.  More q's, more quiet.
    #[clap(
        short,
        long,
        action = Count,
        help = "Turn down logging verbosity (multiple will turn it down more)",
        conflicts_with = "verbose",
    )]
    quiet: u8,
    #[arg(name = "year", short = 'y', long, help = "Specify the year you wish to work with", default_value_t = DEFAULT_YEAR.to_string())]
    year: String,
    #[arg(name = "start_day", short = 's', long, help = "Specify the start day")]
    start_day: Option<String>,
    #[arg(name = "end_day", short = 'e', long, help = "Specify the end day")]
    end_day: Option<String>,
}
