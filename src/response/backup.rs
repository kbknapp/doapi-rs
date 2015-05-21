use std::fmt;

#[derive(Deserialize, Debug)]
pub struct Backup;

impl fmt::Display for Backup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "backup")
    }
}