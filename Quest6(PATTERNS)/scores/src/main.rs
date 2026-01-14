use scores::*;

fn main() {
    println!("{}", score("a"));
    println!("{}", score("Ã£ Ãª Ã?"));
    println!("{}", score("ThiS is A Test"));
}

// $ cargo run
// 1
// 0
// 14
// $
