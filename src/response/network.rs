use std::fmt;

#[derive(Deserialize, Debug)]
pub struct Network;

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "network")
    }
}