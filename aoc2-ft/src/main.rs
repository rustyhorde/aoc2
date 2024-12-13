use std::{fs::OpenOptions, io::Write, path::Path};

use anyhow::Result;
use clap::Parser;
use cli::Args;
use tera::{Context, Tera};

mod cli;

fn main() -> Result<()> {
    // Parse the command line
    let matches = Args::try_parse()?;
    let year = matches.year();
    let start_day = if let Some(day) = matches.start_day() {
        day.parse::<usize>()?
    } else {
        1
    };
    let end_day = if let Some(day) = matches.end_day() {
        day.parse::<usize>()?
    } else {
        25
    };
    let tera = Tera::new("templates/**/*.jinja")?;
    let base_path_str = format!("aoc2-sol/src/year{year}");
    let base_path = Path::new(&base_path_str);

    for day in start_day..=end_day {
        let mut context = Context::new();
        context.insert("day", &format!("{day}"));
        context.insert("year", year);
        context.insert("zero_padded_day", &format!("{day:0>2}"));

        let rendered = tera.render("default_day.jinja", &context)?;

        let day_path = base_path.join(format!("day{day:0>2}.rs"));
        let mut day_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(day_path)?;

        day_file.write_all(rendered.as_bytes())?;
    }
    Ok(())
}
