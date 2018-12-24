extern crate chrono;
extern crate ical;
extern crate regex;
extern crate reqwest;
extern crate slack_api;

#[macro_use]
extern crate serde_json;

mod calendar;
mod slack;
use chrono::{DateTime, Local, Utc};
use regex::Regex;

fn main() {
    let today = Utc::now().date().and_hms(12, 0, 0);
    println!("Today is {}", today.to_rfc3339());

    let off_regex = Regex::new(r"^(Congés|Absent)").unwrap();

    let url = std::env::var("CALENDAR_URL").expect("Env variable 'CALENDAR_URL' not set.");
    let calendar = calendar::get_calendar(url);
    let events = calendar.map(|c| calendar::calendar_to_events(&c));
    let today_events = events.map(|es| calendar::get_today_events(&es, today));
    let off_today_events = today_events.map(|es| calendar::get_off_events(&es, &off_regex));

    match off_today_events {
        Err(e) => panic!("{}", e),
        Ok(es) => {
            let empty = es.len() > 0;
            let names = es.into_iter().map(|e| e.name).collect::<Vec<String>>();
            println!("Today is {}off{}", if empty { "" } else { "NOT " }, if empty { format!(": {:?}", names.join(", ")) } else { String::from("") });
            update_slack_status(&today);
        },
    }
}

fn update_slack_status(today: &DateTime<Utc>) {
    println!("Updating Slack status...");

    let status = "Off today";
    let emoji = ":palm_tree:";

    let token = std::env::var("SLACK_API_TOKEN").expect("Env variable 'SLACK_API_TOKEN' not set.");
    let timeout = today.date().with_timezone(&Local).and_hms(23, 59, 59).with_timezone(&Utc);
    slack::set_status(&token, status, emoji, Some(timeout)).unwrap();

    println!("Done!");
}
