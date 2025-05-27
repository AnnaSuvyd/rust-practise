use std::io::{self, BufRead};

fn convert_time_12_to_24(time_12h: &str) -> String {
    // Формат вхідного часу: "hh:mm:ssAM" або "hh:mm:ssPM"
    let hour: u32 = time_12h[0..2].parse().unwrap();
    let minute = &time_12h[3..5];
    let second = &time_12h[6..8];
    let meridian = &time_12h[8..10]; // "AM" або "PM"

    let hour_24 = match meridian {
        "AM" => {
            if hour == 12 { 0 } else { hour }
        }
        "PM" => {
            if hour == 12 { 12 } else { hour + 12 }
        }
        _ => hour, // На випадок некоректного формату, але за умовою це не потрібно
    };

    format!("{:02}:{:}:{:}", hour_24, minute, second)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let time_12h = lines.next().unwrap().unwrap();
    let time_24h = convert_time_12_to_24(&time_12h);

    println!("{}", time_24h);
}
