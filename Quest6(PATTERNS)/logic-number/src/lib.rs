pub fn number_logic(num: u32) -> bool {
    let parsed = num.to_string();
    let len = parsed.len() as u32;
    let mut sum = 0;


    if len == 1 {
        return true;
    }else {
        for ch in parsed.chars() {
            let n:u32 = ch.to_digit(10).unwrap();
            sum += n.pow(len);
        }
        if sum == num {
            return true;
        }
    }
    return false;
}
