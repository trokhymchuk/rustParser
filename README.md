# iCalendar_parser

* Crates: https://crates.io/crates/iCalendar_parser
* Docs: https://docs.rs/iCalendar_parser/latest/iCalendar_parser/all.html

The iCalendar file contains event with useful information such as begin/end date, invitee, summary and so on.

The parser is inteneded to take that file and parse it into a Rust struct for easier manipulation of the data. The result of the parsed file would be used to determine in which day a particular person have the most events.

*See [ASCIINEMA](https://asciinema.org/a/ktJ8678THq7luDBPz8kG32BTf) of the parser!*

## Field Description

### `BEGIN:VCALENDAR` / `END:VCALENDAR`
- **Description**: These lines enclose the entire iCalendar document. They define the start and end of the calendar content.

### `VERSION`
- **Description**: Specifies the iCalendar version being used. In this case, it's version `2.0`.
- **Example**: `VERSION:2.0`

### `PRODID`
- **Description**: Identifies the product or software that generated the iCalendar file. The value `-//hacksw/handcal//NONSGML v1.0//EN` is typically used as a placeholder for this example, representing a non-standard or generic product ID.
- **Example**: `PRODID:-//hacksw/handcal//NONSGML v1.0//EN`

---

## Event Fields

### `BEGIN:VEVENT` / `END:VEVENT`
- **Description**: These lines enclose an individual event within the calendar. They mark the start and end of the event.

### `UID`
- **Description**: A globally unique identifier for the event. It ensures that this event is uniquely identifiable across different systems.
- **Example**: `UID:a.trokhymchuk@gmail.com`

### `ORGANIZER`
- **Description**: Identifies the person or entity organizing the event. The format includes the name of the organizer (`CN=Artem Trokhymchuk`) and the email address (`MAILTO:a.trokhymchuk@gmail.com`).
- **Example**: `ORGANIZER;CN=John Doe:MAILTO:john.doe@example.com`

### `DTSTART`
- **Description**: The start date and time of the event, given in UTC. The format is `YYYYMMDDTHHMMSSZ`.
- **Example**: `DTSTART:19970714T170000Z` (July 14, 1997, 17:00 UTC)

### `DTEND`
- **Description**: The end date and time of the event, also given in UTC.
- **Example**: `DTEND:19970715T040000Z` (July 15, 1997, 04:00 UTC)

### `SUMMARY`
- **Description**: A short description or title of the event.
- **Example**: `SUMMARY:NaUKMA birthday`

### `GEO`
- **Description**: The geographical coordinates of the event's location. In this case, it represents the latitude and longitude of the event’s location.
- **Example**: `GEO:48.85299,2.36885` (Coordinates for a location in Paris, France)

---

## Summary
These are the core fields present in the provided iCalendar file. The required fields include:
- `BEGIN:VCALENDAR` / `END:VCALENDAR`
- `VERSION`
- `PRODID`
- `BEGIN:VEVENT` / `END:VEVENT`
- `UID`
- `ORGANIZER`
- `DTSTART`
- `DTEND`
- `SUMMARY`
- `GEO`

## Grammar rules

```pest
vc_calendar = { "BEGIN:VCALENDAR" ~ method? ~ ((version  ~ prodid) | (prodid ~ version)) ~ event* ~ "END:VCALENDAR" }
version     = { "VERSION:" ~ float }
prodid      = { "PRODID:" ~ line }
event       = { "BEGIN:VEVENT" ~ uid ~ organizer ~ dtstart ~ dtend ~ summary ~ geo ~ dsc? ~ "END:VEVENT" }
uid         = { "UID:" ~ email_address }
organizer   = { "ORGANIZER;" ~ "CN=" ~ identifier ~ ":" ~ "MAILTO:" ~ email_address }
dtstart     = { "DTSTART:" ~ datetime }
dtend       = { "DTEND:" ~ datetime }
dsc         = { "DESCRIPTION:" ~ line }
summary     = { "SUMMARY:" ~ line }
geo         = { "GEO:" ~ float ~ "," ~ float }

method             = @{ "METHOD:" ~ ("PUBLISH" | "REQUEST") }
email_address      = @{ (letter_or_digit | "." | "_")+ ~ "@" ~ letter_or_digit+ ~ "." ~ letter_or_digit+ }
datetime           = @{ digit{8} ~ "T" ~ digit{6} ~ "Z" }
identifier         = @{ letter_or_digit+ ~ (" " ~ letter_or_digit+)* }
float              = @{ digit+ ~ "." ~ digit+ }
quoted_string      = @{ "\"" ~ (printable_char ~ ANY)* ~ "\"" }
ascii_alphanumeric = @{ ASCII_ALPHANUMERIC }
printable_char     = @{ '\u{20}'..'\u{21}' | '\u{23}'..'\u{5B}' | '\u{5D}'..'\u{7A}' }
digit              = @{ '0'..'9' }
letter_or_digit    = @{ 'a'..'z' | 'A'..'Z' | '0'..'9' }
comment            =  { ";" ~ (ANY ~ "\n")* }
WHITESPACE         = _{ comment | " " | "\t" | "\n" }
line               = @{ (letter_or_digit | " " | "-" | "/" | "." | "," | ":")* }
```