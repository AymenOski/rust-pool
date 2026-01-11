use profanity_filter::*;

fn main() {
    ["hello there", "", "you are stupid", "stupid"]
        .into_iter()
        .for_each(|m| println!("{:?}", check_ms(m)));
}

// $ cargo run
// Ok("hello there")
// Err("ERROR: illegal")
// Err("ERROR: illegal")
// Err("ERROR: illegal")
// $
