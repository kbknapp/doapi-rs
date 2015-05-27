// name    string  The name of the domain itself. This should follow the standard domain format of domain.TLD. For instance, example.com is a valid domain name.
// ttl     number  This value is the time to live for the records on this domain, in seconds. This defines the time frame that clients can cache queried information before a refresh should be requested.
// zone_file   string  This attribute contains the complete contents of the zone file for the selected domain. Individual domain record resources should be used to get more granular control over records. However, this attribute can also be used to get information about the SOA record, which is created automatically and is not accessible as an individual record resource.

use std::fmt;
use std::borrow::Cow;

use response::NamedResponse;

#[derive(Deserialize, Debug)]
pub struct Domain {
    name: String,
    ttl: f64,
    zone_file: String
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Domain:\n\t\
                        name: {}\n\t\
                        ttl: {:.0}\n\t\
                        Zone File: {}\n",
                self.name,
                self.ttl,
                self.zone_file)
    }
}

pub type Domains = Vec<Domain>;

impl NamedResponse for Domain {
    fn name<'a>() -> Cow<'a, str> {
        "domain".into()
    }
}
