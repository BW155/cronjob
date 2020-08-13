extern crate cronjob;

#[cfg(test)]
mod tests {

    use cronjob::CronJob;

    #[test]
    fn test_cronjob() {
        let mut cron = CronJob::new("Test Cron", on_cron);
        cron.offset(2);
        cron.seconds("0-10");
        cron.start_job();
    }

    fn on_cron(name: &str) {
        println!("{}: It's time!", name);
    }
}
