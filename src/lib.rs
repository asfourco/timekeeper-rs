// Library Module for calculations
mod lib {
    use chrono::{Datelike, Duration, Local, NaiveDate};

    pub fn calculate_current_month_burn_rate(
        pay_rate: f32,
        hours_per_day: f32,
        hours_to_date: f32,
    ) {
        let mut burn_rate_hours: f32 = 0.0;
        let mut burn_rate_percentage: f32 = 0.0;

        let total_working_hours: f32 = get_working_hours_in_month(hours_per_day);
        let remaining_working_days: f32 = get_remaining_working_days_in_month() as f32;
        let remaining_working_hours: f32 = remaining_working_days - hours_to_date;
        let total_accumulated: f32 = pay_rate * hours_to_date;

        if remaining_working_days > 0.0 {
            burn_rate_hours = remaining_working_hours / remaining_working_days;
            println!("Your current burn rate is: {}", burn_rate_hours);
        } else {
            println!("You have no more working days left this month");
        }

        if burn_rate_hours > 0.0 {
            burn_rate_percentage = (hours_per_day / burn_rate_hours) * 100.0
        }

        let remaining_hours_percentage: f32 =
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
        let current_date = Local::now();
        let current_month = current_date.month();
        let current_year = current_date.year();

        // Step 2: Get the first day of the current month and the last day of the current month
        let first_day_of_month = NaiveDate::from_ymd_opt(current_year, current_month, 1).expect("Invalid date");


        let last_day_of_month = NaiveDate::from_ymd_opt(current_year, current_month, 1)
            .expect("Invalid date")
            .with_day0(0)
            .unwrap_or_else(|| {
                NaiveDate::from_ymd(current_year, current_month + 1, 1)
            })
            - Duration::days(1);

        // Step 3: Get the number of days in the current month
        let mut current_date = first_day_of_month.clone();
        let mut working_days = 0;

        while current_date <= last_day_of_month {
            if current_date.weekday() != chrono::Weekday::Sat && current_date.weekday() != chrono::Weekday::Sun {
                working_days += 1;
            }
            current_date = current_date + Duration::days(1);
        }
        working_days as u32
    }

    pub fn get_working_hours_in_month(hours_per_day: f32) -> f32 {
        get_working_days_in_month() as f32 * hours_per_day
    }

    pub fn get_remaining_working_days_in_month() -> u32 {
        let current_date = Local::now();
        let current_month = current_date.month();
        let current_year = current_date.year();

        let first_day_of_month = NaiveDate::from_ymd_opt(current_year, current_month, 1).expect("Invalid date");

        let working_days = get_working_days_in_month();

        let today = Local::now().naive_local().date();
        let days_passed = (today - first_day_of_month).num_days() + 1; // Adding 1 to include today

        let remaining_working_days = working_days - days_passed as u32;

        remaining_working_days
    }
}
