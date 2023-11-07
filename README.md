# timekeeper-rs
CLI Tool for freelancers to calculate their burn rate on their budgeted hours

## Usage
```bash
# Get help
$ timekeeper --help
```
There are two commands that can be used: 
- `calendar` to print out the working days and hours for each month in a calendar year
- `burn-rate` to calculate the burn rate of budgeted hours for the current month

To run the `calendar` command:
```bash
$ timekeeper calendar -y {year} -r {hourly rate} --hours_per_day {hours per day, optional}

#########
# Example
# print the total working hours for each month for the year 2023

$ timekeeper calendar  -y 2023
Month: 1 	 Working Days: 22 	 Working Hours: 176
Month: 2 	 Working Days: 20 	 Working Hours: 160
Month: 3 	 Working Days: 23 	 Working Hours: 184
Month: 4 	 Working Days: 20 	 Working Hours: 160
Month: 5 	 Working Days: 23 	 Working Hours: 184
Month: 6 	 Working Days: 22 	 Working Hours: 176
Month: 7 	 Working Days: 21 	 Working Hours: 168
Month: 8 	 Working Days: 23 	 Working Hours: 184
Month: 9 	 Working Days: 21 	 Working Hours: 168
Month: 10 	 Working Days: 22 	 Working Hours: 176
Month: 11 	 Working Days: 22 	 Working Hours: 176
Month: 12 	 Working Days: 21 	 Working Hours: 168
Total Working Days: 260 	 Total Working Hours: 2080.00 	 Average Monthly hours: 173.33
```

To run the `burn-rate` command:
```bash
Usage: timekeeper-rs burn-rate [OPTIONS] --billable-hours <BILLABLE_HOURS> --rate <RATE>

Options:
  -b, --billable-hours <BILLABLE_HOURS>  
  -r, --rate <RATE>                      
      --max-hours <MAX_HOURS>            Override the maximum amount of hours for this month
      --hours_per_day <HOURS_PER_DAY>    Specify hours per day [default: 8.0]
  -h, --help                             Print help
 
#########
# Example:
# calculate burn rate for current month where the max hours for the month
# is 80 hours, and work is budged for 2h/day. So far 30h are billable at a rate of $100

$ timekeeper burn-rate -b 30 -r 100 --max-hours 80 --hours_per_day 2
Your current burn rate (h/day) is: 2.78
Your current burn rate (percentage) is: 0.72 (You are working 28.00 % less than you should be)
You have 50.00 hours left to work this month
You have 62.50 % of your hours left to work this month
You have accumulated $ 3,000.00 so far this month
```

## Building
Use cargo to build the project
```bash
$ cargo build --release
# or
$ make build
```

## Install
```bash
# will build release and install binary to $HOME/.local/bin/
$ make install
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change. If you like the project, please show your appreciation by giving it a star, tweeting about it or referring to this project in your project's readme.

### Legal notice
When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project license.

## Support
If you like this tool consider supporting me by buying me a coffee! https://bit.ly/3u6ex56
