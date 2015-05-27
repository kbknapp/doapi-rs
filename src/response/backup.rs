use std::fmt;
use std::borrow::Cow;

use response::NamedResponse;

#[derive(Deserialize, Debug)]
pub struct Backup;

impl fmt::Display for Backup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "backup")
    }
}

pub type Backups = Vec<Backup>;

impl NamedResponse for Backup {
    fn name<'a>() -> Cow<'a, str> {
        "backup".into()
    }
}
