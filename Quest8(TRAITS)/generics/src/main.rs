use generics::*;

fn main() {
	println!("{}", identity("Hello, world!"));
	println!("{}", identity(3));
}   

// $ cargo run
// Hello, world!
// 3
// $
