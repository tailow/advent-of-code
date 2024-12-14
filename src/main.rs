use std::time::SystemTime;

mod days;

fn main() {
    let start = SystemTime::now();

    days::day13::run();

    match start.elapsed() {
        Ok(elapsed) => {
            println!("Time elapsed: {} µs", elapsed.as_micros());
        }
        _ => {}
    }
}
