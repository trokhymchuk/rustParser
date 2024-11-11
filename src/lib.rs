#![allow(non_snake_case)]

use colored::*;
use pest::Parser;
use pest_derive::Parser;
use std::str::FromStr;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]

pub struct ICalParser;

#[derive(Error, Debug)]
pub enum ICalendarParsingError {
    #[error("Eror parsing iCal file: {0}")]
    ParsingTimeError(String),
}

#[derive(Debug, PartialEq)]
pub struct ICalendar {
    pub version: Option<f64>,
    pub prodid: Option<String>,
    pub events: Vec<Event>,
}

#[derive(Debug, PartialEq)]
pub struct Event {
    pub uid: Option<String>,
    pub organizer: Option<String>,
    pub dtstart: Option<String>,
    pub dtend: Option<String>,
    pub summary: Option<String>,
    pub geo: Option<(f64, f64)>,
    pub description: Option<String>,
}

pub trait PrettyPrint {
    fn pretty_print(&self, colored: Option<bool>);
}

impl ICalendar {
    pub fn parse(ical_text: &str) -> Result<Self, ICalendarParsingError> {
        let mut calendar = ICalendar {
            version: None,
            prodid: None,
            events: Vec::new(),
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

        for event in self.events.iter() {
            event.pretty_print(Some(colored));
        }
    }
}

impl Event {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut event = Event {
            uid: None,
            organizer: None,
            dtstart: None,
            dtend: None,
            summary: None,
            geo: None,
            description: None,
        };

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
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
    }
}
