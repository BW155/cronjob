extern crate cronjob;

#[cfg(test)]
mod tests {

    use cronjob::{CronJob, get_timezone_offset};

    #[test]
    fn it_works() {
        let offset = get_timezone_offset(2);
        let mut cron = CronJob::new("Test Cron", "* * * * * * *", offset, on_cron);
        cron.start_job();
    }

    fn on_cron(name: &str) {
        println!("{}: It's time!", name);
    }
}
