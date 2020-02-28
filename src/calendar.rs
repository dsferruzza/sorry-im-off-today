use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use ical::parser::ical::component::{IcalCalendar, IcalEvent};
use regex::Regex;

pub fn get_calendar(url: &str) -> Result<IcalCalendar, String> {
    let body = reqwest::blocking::get(url)
        .and_then(|r| r.error_for_status())
        .map_err(|e| {
            format!(
                "Error fetching calendar: \"{}\"",
                display_http_error(&e, &url)
            )
        })?
        .text()
        .map_err(|e| format!("Error accessing HTTP response body: {}", e))?;
    ical::IcalParser::new(body.into_bytes().as_slice())
        .next()
        .ok_or("No ICAL calendar was found")?
        .map_err(|e| format!("ICAL parser error: {}", e))
}

fn display_http_error(e: &reqwest::Error, url: &str) -> String {
    e.to_string().replace(format!("{}: ", url).as_str(), "")
}

#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

pub fn calendar_to_events(calendar: &IcalCalendar) -> Vec<Event> {
    calendar
        .events
        .iter()
        .flat_map(|e| parse_event(e).into_iter())
        .collect()
}

fn parse_event(event: &IcalEvent) -> Option<Event> {
    let props = event.properties.iter();
    let confirmed = props
        .clone()
        .any(|p| p.name == "STATUS" && p.value == Some(String::from("CONFIRMED")));
    if confirmed {
        let name = props
            .clone()
            .find(|p| p.name == "SUMMARY")
            .and_then(|p| p.clone().value);
        let start = props
            .clone()
            .find(|p| p.name == "DTSTART")
            .and_then(|p| p.clone().value)
            .as_ref()
            .map(std::string::String::as_str)
            .and_then(parse_date);
        let end = props
            .clone()
            .find(|p| p.name == "DTEND")
            .and_then(|p| p.clone().value)
            .as_ref()
            .map(std::string::String::as_str)
            .and_then(parse_date);
        match (name, start, end) {
            (Some(n), Some(s), Some(e)) => Some(Event {
                name: n,
                start: s,
                end: e,
            }),
            _ => None,
        }
    } else {
        None
    }
}

fn parse_date(date: &str) -> Option<DateTime<Utc>> {
    let format = "%Y%m%dT%H%M%SZ";
    let format2 = "%Y-%m-%dT%H%M%SZ";
    let format3 = "%Y%m%dT%H%M%S";
    let format4 = "%Y%m%d";
    let parsed_date = NaiveDateTime::parse_from_str(date, &format)
        .or_else(|_e| NaiveDateTime::parse_from_str(date, &format2))
        .or_else(|_e| NaiveDateTime::parse_from_str(date, &format3))
        .or_else(|_e| {
            NaiveDate::parse_from_str(date, &format4)
                .map(|nd| nd.and_time(NaiveTime::from_hms(0, 0, 0)))
        })
        .map(|d| DateTime::from_utc(d, Utc));
    if parsed_date.is_err() {
        println!("Cannot parse date {:?} => {:?}", date, parsed_date);
    }
    parsed_date.ok()
}

pub fn get_today_events(events: &[Event], today: DateTime<Utc>) -> Vec<Event> {
    events
        .iter()
        .filter(|e| e.start <= today && e.end >= today)
        .cloned()
        .collect()
}

pub fn get_off_events(events: &[Event], off_regex: &Regex) -> Vec<Event> {
    events
        .iter()
        .filter(|e| off_regex.is_match(&e.name))
        .cloned()
        .collect()
}
