
pub mod util {
    pub fn format_accounting_format(number: f64) -> String {
        let formatted = format!("{:.*}", 2, number);
        let parts: Vec<&str> = formatted.split('.').collect();
        let before_decimal = parts[0]
            .chars()
            .rev()
            .collect::<String>()
            .as_bytes()
            .chunks(3)
            .map(|chunk| unsafe { std::str::from_utf8_unchecked(chunk) })
            .collect::<Vec<&str>>()
            .join(",")
            .chars()
            .rev()
            .collect::<String>();

        if parts.len() > 1 {
            format!("{}.{}", before_decimal, parts[1])
        } else {
            before_decimal
        }
    }
}

pub mod calculator {
    use chrono::{Datelike, Duration, Local, NaiveDate};
    use crate::util::format_accounting_format;

    pub fn calculate_current_month_burn_rate(
        pay_rate: f64,
        hours_per_day: f64,
        hours_to_date: f64,
        max_hours: Option<f64>,
    ) {
        let mut burn_rate_hours: f64 = 0.0;
        let mut burn_rate_percentage: f64 = 0.0;
        let mut total_working_hours: f64 = 0.0;

        if max_hours.is_some() {
            total_working_hours = max_hours.unwrap()
        } else {
            total_working_hours = get_working_hours_in_current_month(hours_per_day);
        }

        let remaining_working_days: f64 = get_remaining_working_days_in_month() as f64;
        let remaining_working_hours: f64 = total_working_hours - hours_to_date;
        let total_accumulated: f64 = pay_rate * hours_to_date;

        if remaining_working_days > 0.0 {
            burn_rate_hours = remaining_working_hours / remaining_working_days;
            println!("Your current burn rate (h/day) is: {:.2}", burn_rate_hours);
        } else {
            println!("You have no more working days left this month");
        }

        if burn_rate_hours > 0.0 {
            burn_rate_percentage = hours_per_day / burn_rate_hours
        }

        let remaining_hours_percentage: f64 =
            (remaining_working_hours / total_working_hours) * 100.0;

        print!("Your current burn rate (percentage) is: {:.2} ", burn_rate_percentage);
        if burn_rate_percentage < 1.0 {
            print!("(You are working {:.2} % less than you should be)", (1.0 - burn_rate_percentage) * 100.0);
        } else if burn_rate_percentage > 1.0 {
            print!("(You are working {:.2} % more than you should be)", (burn_rate_percentage - 1.0) * 100.0);
        } else {
            print!("(You are working exactly as you should be)");
        }
        println!();
        println!(
            "You have {:.2} hours left to work this month",
            remaining_working_hours
        );
        println!(
            "You have {:.2} % of your hours left to work this month",
            remaining_hours_percentage
        );
        println!(
            "You have accumulated $ {} so far this month",
            format_accounting_format(total_accumulated)
        );

    }

    pub fn get_working_days_in_current_month() -> u32 {
        // Step 1: Get current month and year
        let current_date = Local::now().naive_local().date();

        // Step 2: Get the first day of the current month and the last day of the current month
        let first_day_of_month = get_start_day_of_month(current_date.month(), current_date.year());
        let last_day_of_month = get_last_day_of_month(current_date.month(), current_date.year());

        // Step 3: Get the number of days in the current month
        get_working_days_between(first_day_of_month, last_day_of_month)
    }

    pub fn get_working_hours_in_current_month(hours_per_day: f64) -> f64 {
        get_working_days_in_current_month() as f64 * hours_per_day
    }

    pub fn get_start_day_of_month(month: u32, year: i32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date")
    }

    pub fn get_last_day_of_month(month: u32, year: i32) -> NaiveDate {
        if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).expect("Invalid date") - Duration::days(1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1).expect("Invalid date") - Duration::days(1)
        }
    }

    pub fn get_remaining_working_days_in_month() -> u32 {
        let today = Local::now().naive_local().date();
        let days_passed = get_working_days_between(get_start_day_of_month(today.month(), today.year()), today);
        let working_days = get_working_days_in_current_month();


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

pub mod calendar {

    use crate::{calculator, util::format_accounting_format};
    use chrono::{Datelike, Local};
    pub fn print_working_days_in_calendar(year: u32, hours_per_day: f64, rate: Option<f64>){
        /*
        Generate rust code for the following:
       1. for every month in year, get the start date and end date. Using the start date and end date, get the number of working days in the month by calling get_working_days_between()
       2. if year is not defined then year = current year
       3. print the working days for each month in the year and print the total working hours for the month by multiplying the working days with hours_per_day
       4. finally, print the total number of working days and the total working hours in the year
         */

        let mut calendar_year = year as i32;
        let today = Local::now().naive_local().date();
        if calendar_year == 0 {
            calendar_year = today.year();
        }


        let mut total_working_days: u32 = 0;
        let mut total_working_hours: f64 = 0.0;

        for month in 1..=12 {
            let start_date = calculator::get_start_day_of_month(month, calendar_year);
            let end_date = calculator::get_last_day_of_month(month, calendar_year);
            let working_days_in_month = calculator::get_working_days_between(start_date, end_date);
            let working_hours_in_month = working_days_in_month as f64 * hours_per_day;
            total_working_days += working_days_in_month;
            total_working_hours += working_hours_in_month;
            print!("Month: {} \t Working Days: {} \t Working Hours: {}", month, working_days_in_month, working_hours_in_month);
            if rate.is_some() {
                let total_accumulated = rate.unwrap() * working_hours_in_month;
                print!("\t Total Accumulated: $ {}", format_accounting_format(total_accumulated));
            }
            println!();
        }

        print!("Total Working Days: {} \t Total Working Hours: {}", total_working_days, total_working_hours);
        if rate.is_some() {
            print!(" \t Total for Year: $ {}", format_accounting_format(rate.unwrap() * total_working_hours));
        }
        println!();
    }
}

