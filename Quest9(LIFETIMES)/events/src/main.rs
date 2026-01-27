use events::*;
use std::time::Duration;

fn main() {
    println!("{}", Event::Remainder("Go to the doctor").notify());
    println!(
        "{}",
        Event::Registration(Duration::from_secs(49094)).notify()
    );
    println!("{}", Event::Appointment("Go to the doctor").notify());
    println!("{}", Event::Holiday.notify());
}

// $ cargo run
// (Bottom, 50, Go to the doctor) # the message on the last field of the tuple should be printed in a dark gray color (50, 50, 50)
// (Top, 30, You have 13H:38M:14S left before the registration ends) # the message on the last field of the tuple should be printed in a red color (255, 2, 22)
// (Center, 100, Go to the doctor) # the message on the last field of the tuple should be printed in a yellow color (200, 200, 3)
// (Top, 25, Enjoy your holiday) # the message on the last field of the tuple should be printed in a green color (0, 255, 0)
// $