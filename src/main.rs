mod args;
use args::{Arguments, SubCommand};
use clap::Parser;
use simple_logger;
use std::str::FromStr;
use timekeeper_rs::{calculator, calendar};

fn main() {
    let args = Arguments::parse();
    simple_logger::init_with_level(get_log_level(args.verbosity)).unwrap();
    match args.cmd {
        SubCommand::Calendar {
            year,
            rate,
            hours_per_day,
        } => calendar::print_working_days_in_calendar(year, hours_per_day, rate),
        SubCommand::BurnRate {
            billable_hours,
            rate,
            hours_per_day,
            max_hours,
            days_off,
        } => calculator::calculate_current_month_burn_rate(
            rate,
            hours_per_day,
            billable_hours,
            max_hours,
            days_off,
        ),
    }
}

fn get_log_level(verbosity: String) -> log::Level {
    log::Level::from_str(&*verbosity).unwrap_or(log::Level::Info)
}
