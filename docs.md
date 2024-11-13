# .iCalendar file parser

The simple `iCalendar` file parser, that takes textual input and returns it in human-readable format. Also it provide handy api to parse `iCalendar` file and put it into a structure for easier manipulation.

## Supported sections

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


## Installation
```
cargo install iCalendar_parser
```

## Usage

```bash
iCalendar_parser --file tests/iCals/1.ical
```
Would produce:
```
iCalendar File
Version: 2
Product ID: -//hacksw/handcal//NONSGML v1.0//EN
Comment: test comment

Event uid1@example.com
  UID: uid1@example.com
  Organizer: John Doe <john.doe@example.com>
  Start Date: 19970714T170000Z
  End Date: 19970715T040000Z
  Summary: Bastille Day Party
  Location (Geo): 48.85299, 2.36885
```