/// Trait for connecting your method to the `CronJob` object.
pub trait Command: Sync + Send + 'static {
    fn execute(&mut self, name: &str);
}
