# cronjob
A libary for creating cronjobs for your application methods.

How to use the project
======================
<!--Add this to your `Cargo.toml` under `[dependencies]`
``` toml
cronjob = "0.1.0"
```
It's on crates.io now, check it out https://crates.io/crates/cronjob.-->

Currently you have to add this in your `Cargo.toml` under `[dependencies]`
``` toml
cronjob = { git = "https://github.com/TheUberCatman/cronjob.git" }
```

examples
==========
This is an example for the unthreaded version.

```Rust
extern crate cronjob;
use cronjob::{CronJob, get_timezone_offset};

fn main() {
    // Create offset for your required timezone.
    let offset = get_timezone_offset(2);
    // Create the `CronJob` object.
    let mut cron = CronJob::new("Test Cron", "* * * * * * *", offset, on_cron);
    // Start the cronjob
    cron.start_job();
}

// Our cronjob handler
fn on_cron(name: &str) {
    println!("{}: It's time!", name);
}
```

This is an example for the threaded version.

```Rust
extern crate cronjob;
use cronjob::{CronJob, get_timezone_offset};

fn main() {
    // Create offset for your required timezone.
    let offset = get_timezone_offset(2);
    // Create the `CronJob` object.
    let cron = CronJob::new("Test Cron", "* * * * * * *", offset, on_cron);
    // Start the cronjob
    CronJob::start_job_threaded(cron)
}

// Our cronjob handler
fn on_cron(name: &str) {
    println!("{}: It's time!", name);
}
```

If you have any issues, please report.
