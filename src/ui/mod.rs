pub mod boon_log;
pub mod breakbar_log;
pub mod cast_log;
pub mod multi_view;
pub mod scroll;

// TODO: generic log component for reuse?

pub fn format_time(time: i32) -> String {
    format!("{:>3}.{:03}", time / 1000, time.abs() % 1000)
}
