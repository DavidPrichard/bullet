#[macro_use]
extern crate structopt;
extern crate chrono;

use structopt::StructOpt;
use chrono::prelude::*;
use chrono::Duration;

fn main() {
    match Options::from_args() {
        Options::Calendar{ref date} => run_calendar(date)
    }
}

fn run_calendar(start: &Date<FixedOffset>) {
    let days = (0..)
        .map(|offset| *start + Duration::days(offset))
        .take_while(|x| x.month() == start.month());
    
    for x in days {
        println!("* `{}`", x.format("%d %a:"));
    }
}

fn parse_date(s: &str) -> Result<Date<FixedOffset>, String> {
    let full_date = format!("{}-01T00:00:00Z", s);
    DateTime::parse_from_rfc3339(&full_date)
        .map(|x| x.date())
        .map_err(|_| format!("{:?}. Use the format \"YYYY-MM\"", s))
}

#[derive(Debug, StructOpt)]
#[structopt(name="bullet", about="A digital bullet journal page generator")]
enum Options {
    #[structopt(name="monthly-log")]
    /// Generate a list of days for a month
    Calendar {
        #[structopt(parse(try_from_str = "parse_date"))]
        /// Year and month format: YYYY-MM
        date: Date<FixedOffset>
    }
}
