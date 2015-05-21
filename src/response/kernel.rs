use std::fmt;

#[derive(Deserialize, Debug)]
pub struct Kernel;

impl fmt::Display for Kernel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "kernel!")
    }
}