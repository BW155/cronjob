use cron::Schedule;
use chrono::{FixedOffset, Local};

use std::str::FromStr;
use std::thread;
use std::time::Duration;

use command::Command;

/// The object to create and execute cronjobs for yout application.
pub struct CronJob {
    name: String,
    schedule: Schedule,
    offset: FixedOffset,
    pub command: Box<Command>,
}

impl CronJob {
    /// Constructs new `CronJob` object.
    pub fn new<C: Command>(name: &str, expression: &str, offset: FixedOffset, command: C) -> CronJob {

        CronJob {
            name: name.to_string(),
            schedule: Schedule::from_str(expression).expect("Your expression is invalid"),
            offset: offset,
            command: Box::new(command),
        }
    }

    /// Starts the cronjob without threading.
    pub fn start_job(&mut self) {
        loop {
            let upcoming = self.schedule.upcoming(self.offset).take(1);
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
        thread::Builder::new().name(cronjob.name.to_string()).spawn(move || {
            cronjob.start_job();
        }).expect("There was an error in an cronjob");
    }
}
