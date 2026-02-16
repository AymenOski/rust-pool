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
            long_hand: format!("--{}", &name),
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
        let func = self.flags.get(input).ok_or_else(|| "unknown flag".to_string())?;

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

/*
    * Q & A :
    * Q1 : What is the purpose of the `FlagsHandler` struct in this code?
    -A1 : The `FlagsHandler` struct is designed to manage a collection of command line flags and their associated callback functions. It allows you to add flags with their corresponding functions and execute those functions based on user input.
    * Q2 : How does the `add_flag` method work in the `FlagsHandler` struct?
    -A2 : The `add_flag` method takes a `Flag` and a `Callback` function as arguments. It inserts the callback function into the `flags` HashMap using both the short hand and long hand representations of the flag as keys. This allows the handler to recognize both forms of the flag when executing functions.
    * Q3 : What is a slice in Rust `&[]`, and how is it used in the `exec_func` method?
    -A3 : A slice in Rust is a reference to a contiguous sequence of elements in a collection, such as an array or a vector. It does not own the data it references. In the `exec_func` method, a slice of string slices (`&[&str]`) is used to pass the arguments for the callback function. This allows the method to work with a variable number of arguments without taking ownership of them.
    In Rust, a vector (`Vec<T>`) is considered a collection because it is a growable array type that can hold multiple values of the same type. Slices can be created from vectors to provide a view into a portion of the vector without taking ownership of the data.
    * Q4 : deep dive into the `exec_func` method, explain how it retrieves and executes the appropriate callback function based on the input flag.
    -A4 : The `exec_func` method first attempts to retrieve the callback function associated with the input flag from the `flags` HashMap. If the flag is not found, it returns an error indicating that the flag is unknown. If the flag is found, it calls the retrieved callback function, passing the first two elements of the `argv` slice as arguments. The callback function is expected to return a `Result<String, ParseFloatError>`, which the `exec_func` method then maps to a `Result<String, String>` by converting any errors into string messages. Finally, it returns the result of the callback function execution.
    .get(input) retrieves the callback function associated with the input flag from the `flags` HashMap. If the flag is not found, it returns an error indicating that the flag is unknown. If the flag is found, it calls the retrieved callback function.
    .ok_or_else(|| "unknown flag".to_string()) if the flag is found 
    .map_err(|e| e.to_string()) converts any errors from the callback function into string messages, allowing the `exec_func` method to return a consistent error type. if we don't use .map_err, the error type would be ParseFloatError, which is not consistent with the error type returned when the flag is unknown (String).
*/