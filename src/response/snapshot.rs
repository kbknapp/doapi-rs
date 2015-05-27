
use std::fmt;
use std::borrow::Cow;

use response::NamedResponse;

#[derive(Deserialize, Debug)]
pub struct Snapshot;

impl fmt::Display for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "snapshot")
    }
}

pub type Snapshots = Vec<Snapshot>;

impl NamedResponse for Snapshot {
    fn name<'a>() -> Cow<'a, str> {
        "snapshot".into()
    }
}