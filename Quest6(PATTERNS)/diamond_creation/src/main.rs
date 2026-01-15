    use diamond_creation::*;

fn main() {
    println!("{:?}", get_diamond('A'));
    println!("{:?}", get_diamond('C'));
    for line in get_diamond('C') {
        println!("{}", line);
    }
}

// $ cargo run
// ["A"]
// ["  A  ", " B B ", "C   C", " B B ", "  A  "]
//   A  
//  B B 
// C   C
//  B B 
//   A  
// $
