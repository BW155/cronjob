use chrono::{FixedOffset, Local};
use cron::Schedule;

use std::str::FromStr;
use std::thread;
use std::time::Duration;

use command::Command;

/// The object to create and execute cronjobs for yout application.
pub struct CronJob {
    name: String,
    command: Box<dyn Command>,
    seconds: Option<String>,
    minutes: Option<String>,
    hours: Option<String>,
    day_of_month: Option<String>,
    month: Option<String>,
    day_of_week: Option<String>,
    year: Option<String>,
    offset: Option<FixedOffset>,
}

impl CronJob {
    /// Constructs new `CronJob` object.
    pub fn new<C: Command>(name: &str, command: C) -> Self {
        CronJob {
            name: name.to_string(),
            command: Box::new(command),
            seconds: None,
            minutes: None,
            hours: None,
            day_of_month: None,
            month: None,
            day_of_week: None,
            year: None,
            offset: None,
        }
    }

    pub fn seconds(&mut self, seconds: &str) -> &mut Self {
        self.seconds = Some(seconds.to_string());
        self
    }

    pub fn minutes(&mut self, minutes: &str) -> &mut Self {
        self.minutes = Some(minutes.to_string());
        self
    }

    pub fn hours(&mut self, hours: &str) -> &mut Self {
        self.hours = Some(hours.to_string());
        self
    }

    pub fn day_of_month(&mut self, day_of_month: &str) -> &mut Self {
        self.day_of_month = Some(day_of_month.to_string());
        self
    }

    pub fn month(&mut self, month: &str) -> &mut Self {
        self.month = Some(month.to_string());
        self
    }

    pub fn day_of_week(&mut self, day_of_week: &str) -> &mut Self {
        self.day_of_week = Some(day_of_week.to_string());
        self
    }

    pub fn year(&mut self, year: &str) -> &mut Self {
        self.year = Some(year.to_string());
        self
    }

    pub fn offset(&mut self, timezone_offset: i32) -> &mut Self {
        self.offset = Some(FixedOffset::east(timezone_offset));
        self
    }

    /// Returns the schedule for the cronjob, with this you are able to get the next occurences.
    pub fn get_schedule(&mut self) -> Schedule {
        let asterix = String::from("*");
        let cron = format!(
            "{} {} {} {} {} {} {}",
            self.seconds.as_ref().unwrap_or(&asterix),
            self.minutes.as_ref().unwrap_or(&asterix),
            self.hours.as_ref().unwrap_or(&asterix),
            self.day_of_month.as_ref().unwrap_or(&asterix),
            self.month.as_ref().unwrap_or(&asterix),
            self.day_of_week.as_ref().unwrap_or(&asterix),
            self.year.as_ref().unwrap_or(&asterix)
        );
        Schedule::from_str(&cron).unwrap()
    }

    /// Starts the cronjob without threading.
    pub fn start_job(&mut self) {
        let schedule = self.get_schedule();
        let offset = self.offset.unwrap_or_else(|| FixedOffset::east(0));

        loop {
            let mut upcoming = schedule.upcoming(offset).take(1);
            thread::sleep(Duration::from_millis(500));
            let local = &Local::now();

            if let Some(datetime) = upcoming.next() {
                if datetime.timestamp() <= local.timestamp() {
                    self.command.execute(&self.name);
                }
            }
        }
    }

    /// Starts the cronjob with threading. Stops when application quits.
    pub fn start_job_threaded(mut cronjob: CronJob) {
        thread::Builder::new()
            .name(cronjob.name.clone())
            .spawn(move || cronjob.start_job())
            .expect("There was an error in an cronjob");
    }
}
