pub mod boon_log;
pub mod cast_log;
pub mod history;
pub mod multi_view;
pub mod scroll;

pub fn format_time(time: u64) -> String {
    format!("{:>3}.{:03}", time / 1000, time % 1000)
}
