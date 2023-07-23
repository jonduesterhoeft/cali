use chrono::{Local};

pub fn get_local_offset() -> (i32, i32) {
    let local_time = Local::now();
    let offset = local_time.offset();

    let offset_hours = offset.local_minus_utc() / 3600;
    let offset_minutes = (offset.local_minus_utc() % 3600) / 60;

    (offset_hours, offset_minutes)
}