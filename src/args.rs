//! Command line arguments & help
//!
use clap::Parser;

/// Command line tool for tracking work time
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about,
    after_help(
        "\
Arguments:

  <HOURS_WORKED>
        Hours in one of the following formats:

        H:M         h,fr        h.fr

        H = hour    M = minute
        h = hours  fr = fraction of an hour
  <HOURLY_RATE>
        Hourly payment rate as floating point number

  <MAX_HOURS>
        Maximum amount of work hours as integer number
"
    ),
    help_template(
        "\
{before-help}{name} {version} - {about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
Homepage:
    {homepage}

License:
    timekeeper-rs is under MIT license.

Author:
    {authors}"
    )
)]
pub struct Args {
    /// Hours worked so far this month
    #[arg(long = "hours_worked")]
    pub hours_worked: Option<f64>,

    ///specify hourly rate
    #[arg(long = "rate")]
    pub rate: Option<f64>,

    /// Override the maximum amount of hours for this month
    #[arg(long = "max-hours")]
    pub max_hours: Option<u32>,

    /// Specify hours per day
    #[arg(long = "hours_per_day")]
    pub hours_per_day: Option<f64>,
}
