#[macro_use]
extern crate structopt;
extern crate chrono;

use std::fmt;
use structopt::StructOpt;
use chrono::prelude::*;
use chrono::Duration;

fn main() {
    let opt = Options::from_args();
    if let Err(err) = run(&opt) {
        eprintln!("{}", err); 
    }
}

fn run(opt: &Options) -> Result<(), Error> {
    match *opt {
        Options::Calendar{ref date} => run_calendar(date)
    }
}

fn run_calendar(start: &Date<FixedOffset>) -> Result<(), Error> {
    let days = (0..)
        .map(|i| *start + Duration::days(i))
        .take_while(|x| x.month() == start.month());
    
    for x in days {
        println!("* `{}`", x.format("%d %a:"));
    }
    Ok(())
}

fn parse_date(s: &str) -> Result<Date<FixedOffset>, Error> {
    let full_date = format!("{}-01T00:00:00Z", s);
    DateTime::parse_from_rfc3339(&full_date)
        .map(|x| x.date())
        .map_err(|_| Error::InvalidDateString(s))
}

#[derive(Debug, StructOpt)]
#[structopt(name="bullet", about="A digital bullet journal page generator")]
enum Options {
    #[structopt(name="monthly-log", about="Generate a list of days")]
    Calendar {
        #[structopt(parse(try_from_str = "parse_date"))]
        date: Date<FixedOffset>
    }
}

#[derive(Debug)]
enum Error<'a> {
    InvalidDateString(&'a str)
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
            Error::InvalidDateString(s) => 
                write!(f, "{:?}. Use the format \"YYYY-MM\"", s)
       }
    }
}
