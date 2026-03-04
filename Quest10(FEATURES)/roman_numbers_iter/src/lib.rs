use std::collections::BTreeMap;

use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {
        match value {
            0 => RomanNumber(vec![Nulla]),
            _ => {
                let mut num = value;
                let mut digits = Vec::new();
                let roman_nums = BTreeMap::from([
                    (1000, [M].to_vec()),
                    (900, [C, M].to_vec()),
                    (500, [D].to_vec()),
                    (400, [C, D].to_vec()),
                    (100, [C].to_vec()),
                    (90, [X, C].to_vec()),
                    (50, [L].to_vec()),
                    (40, [X, L].to_vec()),
                    (10, [X].to_vec()),
                    (9, [I, X].to_vec()),
                    (5, [V].to_vec()),
                    (4, [I, V].to_vec()),
                    (1, [I].to_vec()),
                ]);

                for (val, digit) in roman_nums.iter().rev() {
                    while num >= *val {
                        for d in digit {
                            digits.push(*d);
                        }
                        num -= *val;
                    }
                }

                RomanNumber(digits)
            }
        }
    }
}
impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let current = roman_to_u32(&self.0);

        let next_num = RomanNumber::from(current + 1);

        *self = next_num.clone();

        Some(next_num)
    }
}

fn roman_to_u32(digits: &[RomanDigit]) -> u32 {
    if digits.len() == 1 && digits[0] == RomanDigit::Nulla {
        return 0;
    }

    fn value(d: RomanDigit) -> u32 {
        match d {
            RomanDigit::Nulla => 0,
            RomanDigit::I => 1,
            RomanDigit::V => 5,
            RomanDigit::X => 10,
            RomanDigit::L => 50,
            RomanDigit::C => 100,
            RomanDigit::D => 500,
            RomanDigit::M => 1000,
        }
    }

    let mut total = 0_u32;
    let mut i = 0_usize;

    while i < digits.len() {
        let v1 = value(digits[i]);

        if i + 1 < digits.len() {
            let v2 = value(digits[i + 1]);

            if v1 < v2 {
                total += v2 - v1;
                i += 2;
                continue;
            }
        }

        total += v1;
        i += 1;
    }

    total
}
