use anyhow::{Context, Result};
use iCalendar_parser::*;
use std::fs;

#[test]
fn test_single_event_valid() {
    let ical_text = load_test_icalendar(1).expect("Failed to load test iCalendar file");
    let calendar = ICalendar::parse(&ical_text).expect("Failed to parse iCalendar content");
    assert_eq!(calendar.version, Some(2.0));
    assert_eq!(
        calendar.prodid,
        Some("-//hacksw/handcal//NONSGML v1.0//EN".to_string())
    );
    assert_eq!(calendar.comments, vec!["test comment"]);
    assert_eq!(calendar.events.len(), 1);
    let event = &calendar.events[0];
    assert_eq!(event.uid, Some("uid1@example.com".to_string()));
    assert_eq!(
        event.organizer,
        Some("John Doe <john.doe@example.com>".to_string())
    );
    assert_eq!(event.dtstart, Some("19970714T170000Z".to_string()));
    assert_eq!(event.dtend, Some("19970715T040000Z".to_string()));
    assert_eq!(event.summary, Some("Bastille Day Party".to_string()));
    assert_eq!(event.geo, Some((48.85299, 2.36885)));
    assert_eq!(event.comments, Vec::<String>::new());
}

#[test]
fn test_multiple_events_valid() {
    // Load the iCalendar file
    let ical_text = load_test_icalendar(2).expect("Failed to load test iCalendar file");

    // Parse the iCalendar content
    let calendar = ICalendar::parse(&ical_text).expect("Failed to parse iCalendar content");

    // Define the expected events
    let expected_events = vec![
            Event {
                uid: Some("a.trokymcnuk@gmail.com".to_string()),
                organizer: Some("Artem Trokhymchuk <a.trokymcnuk@gmail.com>".to_string()),
                dtstart: Some("20241115T090000Z".to_string()),
                dtend: Some("20241115T110000Z".to_string()),
                summary: Some("Daily call".to_string()),
                geo: Some((50.4644775692252, 30.519371783529664)),
                description: Some("Daily call on work".to_string()),
                comments: Vec::new(),
            },
            Event {
                uid: Some("a.trokymcnuk@gmail.com".to_string()),
                organizer: Some("Artem Trokhymchuk <a.trokymcnuk@gmail.com>".to_string()),
                dtstart: Some("20241116T140000Z".to_string()),
                dtend: Some("20241116T160000Z".to_string()),
                summary: Some("Design Review - Website Redesign".to_string()),
                geo: Some((50.4644775692252, 30.519371783529664)),
                description: Some("Design review for the SmartUKMA website.".to_string()),
                comments: Vec::new(),
            },
            Event {
                uid: Some("a.trokymcnuk@gmail.com".to_string()),
                organizer: Some("Artem Trokhymchuk <a.trokymcnuk@gmail.com>".to_string()),
                dtstart: Some("20241117T120000Z".to_string()),
                dtend: Some("20241117T130000Z".to_string()),
                summary: Some("SmartUKMA Sync Call".to_string()),
                geo: Some((50.4644775692252, 30.519371783529664)),
                description: Some("Weekly sync call for the SmartUKMA project to discuss updates, blockers, and next steps.".to_string()),
                comments: Vec::new(),
            },
            Event {
                uid: Some("a.trokymcnuk@gmail.com".to_string()),
                organizer: Some("Artem Trokhymchuk <a.trokymcnuk@gmail.com>".to_string()),
                dtstart: Some("20241118T150000Z".to_string()),
                dtend: Some("20241118T170000Z".to_string()),
                summary: Some("sfsdfsdfsdfsf".to_string()),
                geo: Some((50.4644775692252, 30.519371783529664)),
                description: Some("sdfdfsfsdfsdf".to_string()),
                comments: Vec::new(),
            },
            Event {
                uid: Some("a.trokymcnuk@gmail.com".to_string()),
                organizer: Some("Artem Trokhymchuk <a.trokymcnuk@gmail.com>".to_string()),
                dtstart: Some("20241120T100000Z".to_string()),
                dtend: Some("20241120T120000Z".to_string()),
                summary: Some("I AM CEO Board Meeting - Q4 Review".to_string()),
                geo: Some((50.4644775692252, 30.519371783529664)),
                description: Some("Discues how cool I am as CEO.".to_string()),
                comments: Vec::new(),
            },
        ];
    assert_eq!(calendar.version, Some(2.0));
    assert_eq!(
        calendar.prodid,
        Some("-//hacksw/handcal//NONSGML v1.0//EN".to_string())
    );
    assert_eq!(calendar.events.len(), 5);
    for (parsed_event, expected_event) in calendar.events.iter().zip(expected_events.iter()) {
        assert_eq!(parsed_event, expected_event);
    }
}

// helper
fn load_test_icalendar(n: u32) -> Result<String> {
    let path = format!("./tests/iCals/{}.ical", n); // Path to the iCalendar test file
    let ical_text = fs::read_to_string(path.clone())
        .with_context(|| format!("Failed to read iCalendar file: {}", path))?;
    Ok(ical_text)
}
