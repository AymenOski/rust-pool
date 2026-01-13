use std::{error::Error, fmt::Display};

pub enum ParseErr {
    // expected public fields
}

impl Display for ParseErr {
}

impl Error for ParseErr {
}

pub struct ReadErr {
    // expected public fields
}

impl Display for ReadErr {
}

impl Error for ReadErr {
}