use chrono::Local;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self {
            form_values: (field_name, field_value),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {

        // if the name empty
        if self.name.is_empty() {
            return Err(FormError::new(
                "name",
                self.name.clone(),
                "Username is empty",
            ));
        }

        // if the password is too short
        if self.password.len() < 8 {
            return Err(FormError::new(
                "password", // this is known at compile time, so we can use &'static str
                self.password.clone(), // this is not known at compile time, so we use String
                "Password should be at least 8 characters long",
            ));
        }

        // password complexity
        let has_l = has_letter(&self.password);
        let has_d = has_digit(&self.password);
        let has_s = has_symbol(&self.password);

        if !(has_l && has_d && has_s) {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }

        Ok(())
    }
}

fn has_letter(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_alphabetic())
}

fn has_digit(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_digit())
}

fn has_symbol(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_punctuation())
}

/*
    * Q & A : 
    * Q1 : What does any() do?
    - A1 : The any() method in Rust is an iterator adapter that checks if any element of an iterator satisfies a given condition. It takes a closure as an argument and returns true if at least one element in the iterator returns true for the closure, otherwise it returns false. In the context of the has_letter, has_digit, and has_symbol functions, any() is used to check if there is at least one character in the string that is a letter, digit, or symbol, respectively.
    * Q2 : Why do we use `&'static str` instead of &str for the field name and error message in FormError?
    - A2 : We use `&'static str` for the field name and error message in FormError because these values are known at compile time and will not change during the execution of the program. Using `&'static str` allows us to store string literals directly in the code without needing to allocate memory for them at runtime. This can improve performance and reduce memory usage since the string literals are stored in the binary's read-only memory section. Additionally, using `&'static str` ensures that the references to these strings are valid for the entire duration of the program, preventing potential issues with dangling references or memory leaks.
    * Q3 : The `'static` lifetime?!
    - The `'` symbol is used to denote a lifetime in Rust. In this case, `'static` is a special lifetime that indicates that the reference is valid for the entire duration of the program. This means that the string literal will be stored in a fixed location in memory and can be safely referenced throughout the program without worrying about it being deallocated or going out of scope. The use of lifetimes helps ensure memory safety in Rust by preventing dangling references and ensuring that references are valid for the appropriate duration.
*/