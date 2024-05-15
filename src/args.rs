use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Arguments {
    #[clap(subcommand)]
    pub cmd: SubCommand,

    #[arg(short, long, default_value = "info")]
    pub verbosity: String,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    BurnRate {
        #[arg(short, long)]
        billable_hours: f64,

        #[arg(short, long)]
        rate: f64,

        /// Override the maximum amount of hours for this month
        #[arg(long = "max-hours")]
        max_hours: Option<f64>,

        /// Specify hours per day
        #[arg(long = "hours_per_day", default_value = "8.0")]
        hours_per_day: f64,

        /// Specify days off
        #[arg(long = "days_off", default_value = "0.0")]
        days_off: Option<f64>,
    },
    Calendar {
        #[arg(short, long)]
        year: u32,

        ///specify hourly rate
        #[arg(short, long = "rate")]
        rate: Option<f64>,

        /// Specify hours per day
        #[arg(long = "hours_per_day", default_value = "8.0")]
        hours_per_day: f64,
    },
}
