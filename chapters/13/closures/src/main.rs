use std::thread;
use std::time::Duration;

fn fake_expensive_calculation(intensity: u32) -> u32 {
    println!("Taking fucking forever...");
    thread::sleep(Duration::from_sec(2));
    intensity
}

fn main() {
    let fake_input = 10;
    let fake_rn = 7;

    generate_workout(fake_input, fake_rn);
}
