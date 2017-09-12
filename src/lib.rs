//! # cronjob
//!
//! The `cronjob` library lets you create cronjobs for your methods.
//!
//! ## Getting started
//!
//! ``` no_run
//! extern crate cronjob;
//! use cronjob::CronJob;
//!
//! fn main() {
//!     // Create the `CronJob` object.
//!     let mut cron = CronJob::new("Test Cron", on_cron);
//!     // Start the cronjob.
//!     cron.start_job();
//! }
//!
//! // Our cronjob handler.
//! fn on_cron(name: &str) {
//!     println!("{}: It's time!", name);
//! }
//! ```
//!
//! ## schedule
//! The `cronjob` allows to optionally add schedule parameters.
//!
//! ``` no_run
//! extern crate cronjob;
//! use cronjob::CronJob;
//!
//! fn main() {
//!     // Create the `CronJob` object.
//!     let mut cron = CronJob::new("Test Cron", on_cron);
//!     // Set seconds.
//!     cron.seconds("0");
//!     // Set minutes.
//!     cron.minutes(0);
//!     // Start the cronjob.
//!     cron.start_job();
//! }
//!
//! // Our cronjob handler.
//! fn on_cron(name: &str) {
//!     println!("{}: It's time!", name);
//! }
//! ```
//!
//! ## Threaded
//! ``` no_run
//! extern crate cronjob;
//! use cronjob::CronJob;
//!
//! fn main() {
//!     // Create the `CronJob` object.
//!     let mut cron = CronJob::new("Test Cron Threaded", on_cron);
//!     // start the cronjob.
//!     CronJob::start_job_threaded(cron);
//! }
//!
//! // Our cronjob handler.
//! fn on_cron(name: &str) {
//!     println!("{}: It's time!", name);
//! }
//! ```



extern crate cron;
extern crate chrono;

pub use cronjob::CronJob;

use command::Command;
use chrono::FixedOffset;

mod cronjob;
mod command;

const HOUR: i32 = 3600;

/// Use this to automaticly get an `FixedOffset` for the `CronJob` object
pub fn get_timezone_offset(hours: i32) -> FixedOffset {
    if hours > 0 {
        FixedOffset::east(HOUR * hours)
    } else {
        FixedOffset::west(HOUR * (hours * -1))
    }
}

/// Implementation of `Command` for your methods to be used by the `CronJob` object
impl<T: Sync + Send + 'static + FnMut(&str)> Command for T {
    fn execute(&mut self, name: &str) {
        self(name);
    }
}
