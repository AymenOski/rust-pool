pub enum Security {
    Unknown, // Returns the server URL or panics.
    Message, // Returns the server URL or panics with the error message ERROR: program stops.
    Warning, // Returns the server URL or the message Not found: [MESSAGE], where [MESSAGE] represents the server's error message.
    NotFound, // NotFound: Returns the server URL or the message Not found: [MESSAGE], where [MESSAGE] represents the server's error message.
    UnexpectedUrl, // UnexpectedUrl: Returns the error message or panics with the error message being the server URL.
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(),
        Security::Message =>
            match server {
                Err(url) => panic!("ERROR: program stops"),
                Ok(url) => url.to_string(),
            }
        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),
        Security::NotFound =>
            match server {
                Ok(url) => url.to_string(),
                Err(url) => format!("Not found: {}", url),
            }
        _ =>
            match server {
                Ok(url) => panic!("{}", url),
                Err(url) => url.to_string(),
            }
    }
}
