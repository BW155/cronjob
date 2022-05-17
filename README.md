# cronjob
A libary for creating cronjobs for your application methods.

It's on crates.io now, check it out https://crates.io/crates/cronjob.

How to use the project
======================

Add this to your `Cargo.toml` under `[dependencies]`
``` toml
cronjob = "0.4.17"
```

examples
==========
This is an example for the unthreaded version.

```rust
extern crate cronjob;
use cronjob::CronJob;

fn main() {
    // Create the `CronJob` object.
    let mut cron = CronJob::new("Test Cron", on_cron);
    // Set to fire when seconds is 0, 2 or 4
    cron.seconds("0,2,4");
    // Set to fire when day of week is Monday or Friday
    cron.day_of_week("Mon,Fri");
    // Set offset for UTC
    cron.offset(0);
    // Start the cronjob
    cron.start_job();
}
    
// Our cronjob handler
fn on_cron(name: &str) {
    println!("{}: It's time!", name);
}
```

This is an example for the threaded version.

```rust
extern crate cronjob;
use cronjob::CronJob;

fn main() {
    // Create the `CronJob` object.
    let cron = CronJob::new("Test Cron", on_cron);
    // Set to fire when seconds is 0
    cron.seconds("0");
    // Set offset for UTC
    cron.offset(0);
    // Start the cronjob
    CronJob::start_job_threaded(cron)
}
    
// Our cronjob handler
fn on_cron(name: &str) {
    println!("{}: It's time!", name);
}
```

If you have any issues, please report.
