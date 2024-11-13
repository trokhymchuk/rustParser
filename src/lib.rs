#![allow(non_snake_case)]

use colored::*;
use pest::Parser;
use pest_derive::Parser;
use std::str::FromStr;
use thiserror::Error;

/// The `ICalParser` struct is the main parser for the iCalendar format.
#[derive(Parser)]
#[grammar = "./grammar.pest"]

pub struct ICalParser;

/// Enum representing errors during PARSING of the iCal fiel.
#[derive(Error, Debug)]
pub enum ICalendarParsingError {
    #[error("Eror parsing iCal file: {0}")]
    ParsingTimeError(String),
}

/// Struct representing an iCalendar file.
#[derive(Debug, PartialEq)]
pub struct ICalendar {
    /// The version of the iCalendar format used.
    pub version: Option<f64>,
    /// The product ID identifying the software that created the iCalendar file.
    pub prodid: Option<String>,
    /// A list of events contained in the iCalendar file.
    pub events: Vec<Event>,
    /// A list of comments found in the iCalendar file.
    pub comments: Vec<String>,
}

/// Struct representing a single event in the iCalendar file.
#[derive(Debug, PartialEq)]
pub struct Event {
    /// Unique identifier for the event.
    pub uid: Option<String>,
    /// Organizer of the event.
    pub organizer: Option<String>,
    /// The start date and time of the event.
    pub dtstart: Option<String>,
    /// The end date and time of the event.
    pub dtend: Option<String>,
    /// The summary or title of the event.
    pub summary: Option<String>,
    /// The geographical location of the event.
    pub geo: Option<(f64, f64)>,
    /// A detailed description of the event.
    pub description: Option<String>,
    /// A list of comments associated with the event.
    pub comments: Vec<String>,
}

/// Trait to allow pretty printing to the *stdout* (pls note that stdout is only supported output).
pub trait PrettyPrint {
    /// Prints the iCalendar or event in a human-readable format with optional colorization.
    ///
    /// # Arguments
    ///
    /// * `colored` - A boolean flag to enable or disable colorized output. If `None`, colorization is enabled by default.
    fn pretty_print(&self, colored: Option<bool>);
}

impl ICalendar {
    /// Parses an iCalendar string and returns an `ICalendar` struct.
    ///
    /// # Arguments
    ///
    /// * `ical_text` - A string containing the texual representation of the iCalendar.
    ///
    /// # Returns
    ///
    /// * A result containing either the parsed `ICalendar` object or an `ICalendarParsingError`.
    pub fn parse(ical_text: &str) -> Result<Self, ICalendarParsingError> {
        let mut calendar = ICalendar {
            version: None,
            prodid: None,
            events: Vec::new(),
            comments: Vec::new(),
        };

        let pairs_unfolded = ICalParser::parse(Rule::vc_calendar, ical_text);
        let parsed_tokens = match pairs_unfolded {
            Ok(parsed_ical) => parsed_ical,
            Err(error) => return Err(ICalendarParsingError::ParsingTimeError(error.to_string())),
        };

        log::debug!("Parsed pairs: {:?}", parsed_tokens);

        for top_lvl_token in parsed_tokens {
            let inner = top_lvl_token.into_inner();
            for inner_token in inner {
                log::debug!("Got pair type: {:?}", inner_token.as_rule());
                match inner_token.as_rule() {
                    Rule::comment => {
                        let comment_str = inner_token
                            .as_str()
                            .trim_start_matches(';')
                            .trim()
                            .to_string();
                        calendar.comments.push(comment_str);
                    }
                    Rule::version => {
                        let version_str = inner_token.as_str().trim_start_matches("VERSION:");
                        calendar.version = Some(version_str.parse().unwrap());
                    }
                    Rule::prodid => {
                        let prodid_str = inner_token.as_str().trim_start_matches("PRODID:");
                        calendar.prodid = Some(prodid_str.to_string());
                    }
                    Rule::event => {
                        let event = Event::parse(inner_token);
                        log::debug!("Parsed event: {:?}", event);
                        calendar.events.push(event);
                    }
                    _ => {}
                }
            }
        }
        Ok(calendar)
    }
}

impl PrettyPrint for ICalendar {
    fn pretty_print(&self, colored: Option<bool>) {
        let colored = colored.unwrap_or(true);

        if colored {
            println!("{}", "iCalendar File".bold().underline().cyan());
        } else {
            println!("iCalendar File");
        }

        if let Some(version) = self.version {
            if colored {
                println!("{}: {}", "Version".yellow(), version);
            } else {
                println!("Version: {}", version);
            }
        }
        if let Some(prodid) = &self.prodid {
            if colored {
                println!("{}: {}", "Product ID".yellow(), prodid);
            } else {
                println!("Product ID: {}", prodid);
            }
        }
        for comment in &self.comments {
            if colored {
                println!("{}", format!("Comment: {}", comment).yellow());
            } else {
                println!("Comment: {}", comment);
            }
        }
        for event in self.events.iter() {
            event.pretty_print(Some(colored));
        }
    }
}

impl Event {
    /// Parses a single event from the iCalendar data.
    ///
    /// # Arguments
    ///
    /// * `pair` - The pairs from the parsing of the iCalendar text file.
    ///
    /// # Returns
    ///
    /// * The parsed `Event` struct containing the event data.
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut event = Event {
            uid: None,
            organizer: None,
            dtstart: None,
            dtend: None,
            summary: None,
            geo: None,
            description: None,
            comments: Vec::new(),
        };

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::comment => {
                    let comment_str = inner_pair
                        .as_str()
                        .trim_start_matches(';')
                        .trim()
                        .to_string();
                    event.comments.push(comment_str); // Store the comment in the event
                }
                Rule::uid => {
                    event.uid = Some(inner_pair.as_str().trim_start_matches("UID:").to_string());
                }
                Rule::organizer => {
                    let organizer_str = inner_pair.as_str().trim_start_matches("ORGANIZER;CN=");
                    let organizer_details: Vec<&str> = organizer_str.split(":MAILTO:").collect();
                    if organizer_details.len() == 2 {
                        event.organizer = Some(format!(
                            "{} <{}>",
                            organizer_details[0], organizer_details[1]
                        ));
                    }
                }
                Rule::dtstart => {
                    event.dtstart = Some(
                        inner_pair
                            .as_str()
                            .trim_start_matches("DTSTART:")
                            .to_string(),
                    );
                }
                Rule::dtend => {
                    event.dtend =
                        Some(inner_pair.as_str().trim_start_matches("DTEND:").to_string());
                }
                Rule::summary => {
                    event.summary = Some(
                        inner_pair
                            .as_str()
                            .trim_start_matches("SUMMARY:")
                            .to_string(),
                    );
                }
                Rule::geo => {
                    let geo_str = inner_pair.as_str().trim_start_matches("GEO:");
                    let coords: Vec<&str> = geo_str.split(',').collect();
                    if coords.len() == 2 {
                        if let (Ok(lat), Ok(lon)) =
                            (f64::from_str(coords[0]), f64::from_str(coords[1]))
                        {
                            event.geo = Some((lat, lon));
                        }
                    }
                }
                Rule::dsc => {
                    event.description = Some(
                        inner_pair
                            .as_str()
                            .trim_start_matches("DESCRIPTION:")
                            .to_string(),
                    );
                }

                _ => {}
            }
        }
        event
    }
}

impl PrettyPrint for Event {
    fn pretty_print(&self, colored: Option<bool>) {
        let colored = colored.unwrap_or(true);

        if colored {
            println!(
                "\n{} {}",
                "Event".bold().underline().blue(),
                self.uid.as_deref().unwrap_or("Unknown")
            );
        } else {
            println!("\nEvent {}", self.uid.as_deref().unwrap_or("Unknown"));
        }

        if let Some(uid) = &self.uid {
            if colored {
                println!("  {}: {}", "UID".green(), uid);
            } else {
                println!("  UID: {}", uid);
            }
        }
        if let Some(organizer) = &self.organizer {
            if colored {
                println!("  {}: {}", "Organizer".green(), organizer);
            } else {
                println!("  Organizer: {}", organizer);
            }
        }
        if let Some(dtstart) = &self.dtstart {
            if colored {
                println!("  {}: {}", "Start Date".green(), dtstart);
            } else {
                println!("  Start Date: {}", dtstart);
            }
        }
        if let Some(dtend) = &self.dtend {
            if colored {
                println!("  {}: {}", "End Date".green(), dtend);
            } else {
                println!("  End Date: {}", dtend);
            }
        }
        if let Some(summary) = &self.summary {
            if colored {
                println!("  {}: {}", "Summary".green(), summary);
            } else {
                println!("  Summary: {}", summary);
            }
        }
        if let Some((lat, lon)) = self.geo {
            if colored {
                println!("  {}: {}, {}", "Location (Geo)".green(), lat, lon);
            } else {
                println!("  Location (Geo): {}, {}", lat, lon);
            }
        }
        if let Some(description) = &self.description {
            if colored {
                println!("  {}: {}", "Description".green(), description);
            } else {
                println!("  Description: {}", description);
            }
        }
        for comment in &self.comments {
            if colored {
                println!("{}", format!("  Comment: {}", comment).yellow());
            } else {
                println!("  Comment: {}", comment);
            }
        }
    }
}
