pub fn spell(n: u64) -> String {
    if n == 1_000_000 {
        return "one million".to_string();
    }

    fn helper(n: u64) -> String {
        let below_20 = ["one","two","three","four","five","six","seven","eight","nine",
                        "ten","eleven","twelve","thirteen","fourteen","fifteen",
                        "sixteen","seventeen","eighteen","nineteen"];

        let tens = ["twenty","thirty","forty","fifty","sixty","seventy","eighty","ninety"];

        match n {
            1..=19 => below_20[(n-1) as usize].to_string(),
            20..=99 => {
                let t = tens[(n/10 - 2) as usize];
                if n % 10 == 0 {
                    t.to_string()
                } else {
                    format!("{}-{}", t, helper(n%10))
                }
            },
            100..=999 => {
                let rem = n % 100;
                if rem == 0 {
                    format!("{} hundred", helper(n/100))
                } else {
                    format!("{} hundred {}", helper(n/100), helper(rem))
                }
            },
            1000..=999_999 => {
                let rem = n % 1000;
                if rem == 0 {
                    format!("{} thousand", helper(n/1000))
                } else {
                    format!("{} thousand {}", helper(n/1000), helper(rem))
                }
            },
            _ => "".to_string()
        }
    }

    helper(n)
}
