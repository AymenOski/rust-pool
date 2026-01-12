use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl<'a> Flag {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        Flag{
            short_hand: format!("-{}", &name.chars().next().unwrap()),
            long_hand: format!("--{}", name),
            desc: d.to_string()
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

     pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let func = self
            .flags
            .get(input)
            .ok_or_else(|| "unknown flag".to_string())?;

        let result = func(argv[0], argv[1]).map_err(|e| e.to_string())?;

        Ok(result)
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f64 = a.parse()?;
    let y: f64 = b.parse()?;
    let res = x / y;
    Ok(res.to_string())
}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f64 = a.parse()?;
    let y: f64 = b.parse()?;
    let res = x % y;
    Ok(res.to_string())
}
