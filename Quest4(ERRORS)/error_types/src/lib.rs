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
                "password",
                self.password.clone(),
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