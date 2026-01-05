use std::io;

fn main(){
    let riddle: &str = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer: &str = "The letter e";
    let mut cmp = 0;
    let mut input = String::new();

    loop {
        println!("{}", riddle);
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        cmp += 1;
        
        if input.trim() == answer {
            println!("Number of trials: {}",cmp);
            break;
        }
    }
}
