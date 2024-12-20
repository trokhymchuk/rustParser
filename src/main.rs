#![allow(non_snake_case)]
use anyhow::{self, Context};
use clap::{Arg, Command};
use iCalendar_parser::*;
use std::fs;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("iCal Parser")
        .version("0.1.4")
        .author("Artem Trokhymchuk")
        .about("Parse and display iCalendar files")
        .long_about(
            "iCal Parser is a command-line tool designed to parse iCalendar files (.iCal) and display the events contained in them. \n\n
Usage Examples:\n
1. Parse an iCalendar file and display events with colored output (default behavior):\n
   $ iCal-parser -f tests/iCals/1.ical\n
2. Parse an iCalendar file and display events without color:\n
   $ iCal-parser -f tests/iCals/1.ical --no-color true\n")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Path to the iCalendar file to parse")
                .value_parser(clap::value_parser!(String))
                .required(true),
        )
        .arg(
            Arg::new("no-color")
                .long("no-color")
                .help("Disables colored output")
                .value_parser(clap::value_parser!(bool)),
        )
        .get_matches();

    let file_path = matches
        .get_one::<String>("file")
        .context("File path is required")?;

    let ical_text = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    let calendar = ICalendar::parse(&ical_text).context("Failed to parse iCalendar content")?;
    let colored = !matches.contains_id("no-color");

    calendar.pretty_print(Some(colored));
    calendar.print_most_busy_day(Some(colored));
    Ok(())
}
