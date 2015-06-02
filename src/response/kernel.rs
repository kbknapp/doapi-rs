// "kernel": {
//       "id": 2233,
//       "name": "Ubuntu 14.04 x64 vmlinuz-3.13.0-37-generic",
//       "version": "3.13.0-37-generic"
//     },

use std::fmt;
use std::borrow::Cow;

use response::NamedResponse;

#[derive(Deserialize, Debug)]
pub struct Kernel {
    pub id: String,
    pub name: String,
    pub version: String,
}

impl fmt::Display for Kernel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "kernel!")
    }
}

impl NamedResponse for Kernel {
    fn name<'a>() -> Cow<'a, str> {
        "kernel".into()
    }
}

pub type Kernels = Vec<Kernel>;
