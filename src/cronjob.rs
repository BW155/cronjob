use cron::Schedule;
use chrono::{FixedOffset, Local};

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

    pub fn offset(&mut self, timezone_offset: i64) -> &mut Self {
        self.offset = Some(FixedOffset::east(timezone_offset as i32));
        self
    }

    fn get_schedule(&mut self) -> Schedule {
        let cron = format!("{} {} {} {} {} {} {}",
                           self.seconds.clone().unwrap_or("*".to_string()),
                           self.minutes.clone().unwrap_or("*".to_string()),
                           self.hours.clone().unwrap_or("*".to_string()),
                           self.day_of_month.clone().unwrap_or("*".to_string()),
                           self.month.clone().unwrap_or("*".to_string()),
                           self.day_of_week.clone().unwrap_or("*".to_string()),
                           self.year.clone().unwrap_or("*".to_string()));
        Schedule::from_str(&cron).unwrap()
    }

    /// Starts the cronjob without threading.
    pub fn start_job(&mut self) {
        let schedule = self.get_schedule();
        let offset = self.offset.unwrap_or(FixedOffset::east(0));
        loop {
            let upcoming = schedule.upcoming(offset).take(1);
            thread::sleep(Duration::new(1, 0));
            for datetime in upcoming {
                let local = &Local::now();
                if datetime.timestamp() <= local.timestamp() {
                    self.command.execute(&self.name);
                }
            }
        }
    }

    /// Starts the cronjob with threading. Stops when application quits.
    pub fn start_job_threaded(mut cronjob: CronJob) {
        thread::Builder::new()
            .name(cronjob.name.to_string())
            .spawn(move || { cronjob.start_job(); })
            .expect("There was an error in an cronjob");
    }
}
