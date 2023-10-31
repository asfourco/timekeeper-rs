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

# Example
$ timekeeper calendar -y 2021 -r 100 --hours_per_day 7.5
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
 
 # Example
 $ timekeeper burn-rate -b 120 -r 100 --max-hours 140
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
# will build release and install to /usr/local/bin
$ make install
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change. If you like the project, please show your appreciation by giving it a star, tweeting about it or referring to this project in your project's readme.

### Legal notice
When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project license.
