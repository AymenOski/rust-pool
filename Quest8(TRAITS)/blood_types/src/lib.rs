use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_uppercase();

        if s.len() < 1 || s.len() > 3 {
            return Err(());
        }

        let last_char = s.chars().last().unwrap();
        let rh = match last_char {
            '+' => RhFactor::Positive,
            '-' => RhFactor::Negative,
            _ => return Err(()),
        };

        let antigen_str = &s[..s.len() - 1];

        let antigen = match antigen_str {
            "A" => Antigen::A,
            "B" => Antigen::B,
            "AB" => Antigen::AB,
            "O" => Antigen::O,
            _ => return Err(()),
        };

        Ok(BloodType {
            antigen,
            rh_factor: rh,
        })
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };

        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{}{}", antigen_str, rh_str)
    }
}

impl BloodType {
    pub fn can_receive_from(self, other: Self) -> bool {
        let abo_ok = match (self.antigen, other.antigen) {
            (Antigen::O, Antigen::O) => true,
            (Antigen::O, _) => false,

            (Antigen::A, Antigen::A) | (Antigen::A, Antigen::O) => true,

            (Antigen::B, Antigen::B) | (Antigen::B, Antigen::O) => true,

            (Antigen::AB, _) => true,
            _ => false,
        };

        let rh_ok = match (self.rh_factor, other.rh_factor) {
            (RhFactor::Negative, RhFactor::Positive) => false,
            _ => true,
        };

        abo_ok && rh_ok
    }

    pub fn donors(self) -> Vec<Self> {
        let mut result = Vec::new();

        for &antigen in &[Antigen::A, Antigen::B, Antigen::AB, Antigen::O] {
            for &rh in &[RhFactor::Positive, RhFactor::Negative] {
                let candidate = BloodType { antigen, rh_factor: rh };
                if self.can_receive_from(candidate) {
                    result.push(candidate);
                }
            }
        }

        result
    }

    pub fn recipients(self) -> Vec<Self> {
        let mut result = Vec::new();

        for &antigen in &[Antigen::A, Antigen::B, Antigen::AB, Antigen::O] {
            for &rh in &[RhFactor::Positive, RhFactor::Negative] {
                let candidate = BloodType { antigen, rh_factor: rh };
                if candidate.can_receive_from(self) {
                    result.push(candidate);
                }
            }
        }

        result
    }
}