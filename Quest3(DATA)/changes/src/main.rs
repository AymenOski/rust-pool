//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

use changes::*;

fn main() {
    let mut lights = ["living_room", "bedroom", "rest_room"].map(Light::new);
    println!("{:?}", lights);
    // println!("brightness = {}", lights[0].brightness);

    change_brightness(&mut lights, "living_room", 200);

    // println!("new brightness = {}", lights[0].brightness);
}

/*
    * Q & A :
    * Q1 : what does the lights array contain after the first line of main() is executed?
    - A1 : instead of just having an array of strings, we now have an array of `Light` structs.
    -> [Light { alias: "living_room", brightness: 0 }, Light { alias: "bedroom", brightness: 0 }, Light { alias: "rest_room", brightness: 0 }]
    the map simply takes each string in the array and creates a `Light` struct with that string as the alias and a default brightness of 0.
*/
