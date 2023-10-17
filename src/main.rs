mod args;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("Welcome to your timekeeper app");
    println!("Args: {:?}", args);
    let _hours_worked = args.hours_worked.unwrap_or(0.0);
    let _rate = args.rate.unwrap_or(0.0);
    let _max_hours = args.max_hours.unwrap_or(0);
    let _hours_per_day = args.hours_per_day.unwrap_or(8.0);


}
