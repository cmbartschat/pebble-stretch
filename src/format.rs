use pebble_rust_2026::LocalTime;

pub fn get_digits(time: &LocalTime) -> (i32, i32, i32, i32) {
    let mut hour = time.hour();
    if hour == 0 {
        hour = 12;
    } else if hour > 12 {
        hour -= 12;
    }

    (
        hour.div_euclid(10),
        hour.rem_euclid(10),
        time.minute().div_euclid(10),
        time.minute().rem_euclid(10),
    )
}
