use pest:: Parser;
use iCalendar_parser::*;

#[test]
fn test1() {
    assert_eq!(format!("{:?}", Grammar::parse(Rule::float, "512.256")), "Ok([Pair { rule: float, span: Span { str: \"512.256\", start: 0, end: 7 }, inner: [] }])");
}
