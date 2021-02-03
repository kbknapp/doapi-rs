// droplet_id           number      The affected droplet's ID
// date_of_migration    string      A time value given in ISO8601 combined date
// and time format
// that represents when the migration will
// occur for the droplet.
// url                  string      A URL pointing to the Droplet's API
// endpoint. This is the
// endpoint to be used if you want to retrieve
// information about
//                                  the droplet.

use std::fmt;
use std::borrow::Cow;

use crate::response::{self, NamedResponse};

#[derive(Deserialize, Debug)]
pub struct DropletUpgrade {
    droplet_id: f64,
    date_of_migration: String,
    url: String,
}

impl response::NotArray for DropletUpgrade {}

impl fmt::Display for DropletUpgrade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Droplet ID: {:.0}\n\
                  Date of Migration: {}\n\
                  URL: {}",
               self.droplet_id,
               self.date_of_migration,
               self.url)

    }
}

impl NamedResponse for DropletUpgrade {
    fn name<'a>() -> Cow<'a, str> { "upgrade".into() }
}

pub type DropletUpgrades = Vec<DropletUpgrade>;

pub type ResponseStringArray = Vec<String>;
