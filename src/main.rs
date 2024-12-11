use std::time::SystemTime;

mod days;

fn main() {
    let start = SystemTime::now();

    days::day11::run();

    match start.elapsed() {
        Ok(elapsed) => {
            println!("Time elapsed: {} ms", elapsed.as_millis());
        }
        _ => {}
    }
}
