// Library Module for calculations
pub mod calculator {
    use chrono::{Datelike, Duration, Local, NaiveDate};

    pub fn calculate_current_month_burn_rate(
        pay_rate: f64,
        hours_per_day: f64,
        hours_to_date: f64,
    ) {
        let mut burn_rate_hours: f64 = 0.0;
        let mut burn_rate_percentage: f64 = 0.0;

        let total_working_hours: f64 = get_working_hours_in_month(hours_per_day);
        let remaining_working_days: f64 = get_remaining_working_days_in_month() as f64;
        let remaining_working_hours: f64 = total_working_hours - hours_to_date;
        let total_accumulated: f64 = pay_rate * hours_to_date;

        if remaining_working_days > 0.0 {
            burn_rate_hours = remaining_working_hours / remaining_working_days;
            println!("Your current burn rate is: {}", burn_rate_hours);
        } else {
            println!("You have no more working days left this month");
        }

        if burn_rate_hours > 0.0 {
            burn_rate_percentage = (hours_per_day / burn_rate_hours) * 100.0
        }

        let remaining_hours_percentage: f64 =
            (remaining_working_hours / total_working_hours) * 100.0;

        println!("Your current burn rate is: {}%", burn_rate_percentage);
        println!(
            "You have {} hours left to work this month",
            remaining_working_hours
        );
        println!(
            "You have {}% of your hours left to work this month",
            remaining_hours_percentage
        );
        println!(
            "You have accumulated {} so far this month",
            total_accumulated
        );
    }

    pub fn get_working_days_in_month() -> u32 {
        // Step 1: Get current month and year
        let current_date = Local::now().naive_local().date();

        // Step 2: Get the first day of the current month and the last day of the current month
        let first_day_of_month = get_start_day_of_month(current_date);
        let last_day_of_month = get_last_day_of_month(current_date);

        // Step 3: Get the number of days in the current month
        get_working_days_between(first_day_of_month, last_day_of_month)
    }

    pub fn get_working_hours_in_month(hours_per_day: f64) -> f64 {
        get_working_days_in_month() as f64 * hours_per_day
    }

    pub fn get_start_day_of_month(today: NaiveDate) -> NaiveDate {
        let current_month = today.month();
        let current_year = today.year();
        NaiveDate::from_ymd_opt(current_year, current_month, 1).expect("Invalid date")
    }

    pub fn get_last_day_of_month(today: NaiveDate) -> NaiveDate {
        let current_month = today.month();
        let current_year = today.year();

        NaiveDate::from_ymd_opt(current_year, current_month + 1, 1).expect("Invalid date") - Duration::days(1)
    }

    pub fn get_remaining_working_days_in_month() -> u32 {
        let today = Local::now().naive_local().date();
        let days_passed = get_working_days_between(get_start_day_of_month(today), today);
        let working_days = get_working_days_in_month();


        let remaining_working_days = working_days - days_passed;
        remaining_working_days
    }

    pub fn get_working_days_between(start_date: NaiveDate, end_date: NaiveDate) -> u32{
        let mut current_date = start_date.clone();
        let mut working_days: u32 = 0;

        while current_date <= end_date {
            if current_date.weekday() != chrono::Weekday::Sat && current_date.weekday() != chrono::Weekday::Sun {
                working_days += 1;
            }
            current_date = current_date + Duration::days(1);
        }
        working_days
    }
}
