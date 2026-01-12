use cipher::*;

fn main() {
    println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
    println!("{:?}", cipher("1Hello 2world!", "svool"));
}

// $ cargo run
// Ok(())
// Err(CipherError { expected: "1Svool 2dliow!" })
// $