// droplet_limit    number      The total number of droplets the user may have
// email            string      The email the user has registered for Digital
// Ocean with
// uuid             string      The universal identifier for this user
// email_verified   boolean     If true, the user has verified their account
// via email. False otherwise.

use std::fmt;
use std::borrow::Cow;

use response::NamedResponse;
use response::NotArray;

#[derive(Deserialize, Debug)]
pub struct Account {
    /// droplet_limit is a "number" in json, which could be a float, even thought that's not a
    /// reasonable value for a droplet limit, neither is a negative number
    pub droplet_limit: f64,
    pub email: String,
    pub uuid: String,
    pub email_verified: bool,
}

impl NotArray for Account {}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Email: {}\n\
                   Droplet Limit: {:.0}\n\
                   UUID: {}\n\
                   E-Mail Verified: {}",
               self.email,
               self.droplet_limit,
               self.uuid,
               self.email_verified)
    }
}

impl NamedResponse for Account {
    fn name<'a>() -> Cow<'a, str> { "account".into() }
}

// TODO: Implement response headers:
// content-type: application/json; charset=utf-8
// status: 200 OK
// ratelimit-limit: 1200
// ratelimit-remaining: 1137
// ratelimit-reset: 1415984218
