mod logger;
mod args;

use clap::Parser;
use args::{Arguments, SubCommand};
use logger::DummyLogger;
use timekeeper_rs::{calculator, calendar};

fn main() {
    let args = Arguments::parse();
    let logger = DummyLogger::new(args.verbosity);
    match args.cmd {
        SubCommand::Calendar { year, rate, hours_per_day } => calendar::print_working_days_in_calendar(year, hours_per_day, rate) ,
        SubCommand::BurnRate { billable_hours, rate, hours_per_day, max_hours } => calculator::calculate_current_month_burn_rate(rate, hours_per_day, billable_hours, max_hours),
        _ => logger.log("No command specified"),
    }
}
