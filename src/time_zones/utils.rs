pub fn seconds_to_string(seconds: i32) -> String {
    let sign = if seconds > 0 {
        '+'
    } else if seconds < 0 {
        '-'
    } else {
        ' '
    };

    let value = seconds.abs();
    let hours = value / 3600;
    let remains = hours * 3600;

    let mins = value - remains;

    if mins < 10 {
        format!("UTC{}{}:0{}", sign, hours, mins)
    } else {
        format!("UTC{}{}:{}", sign, hours, mins)
    }
}
