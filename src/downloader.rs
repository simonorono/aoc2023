use std::cmp::min;

const MAX_DAY: i32 = 25;

pub fn download_input(session_cookie: String, until_day: Option<i32>) {
    let max_day = match until_day {
        Some(day) => min(day, MAX_DAY),
        None => MAX_DAY,
    };

    println!(
        "Downloading with cookie {} and until day {}",
        session_cookie, max_day
    )
}
