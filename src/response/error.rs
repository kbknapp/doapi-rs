use std::fmt;

// example: Missing action
// {"id":"not_found","message":"The resource you were accessing could not be found."}
#[derive(Deserialize, Debug)]
pub struct DoError {
    id: String,
    message: String
}

impl fmt::Display for DoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DigitalOcean Error Response: {}",
                self.message)
    }
}
