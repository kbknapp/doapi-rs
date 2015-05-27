use std::borrow::Cow;
use std::fmt;

use response::NamedResponse;

#[derive(Deserialize)]
pub struct DnsRecord;

pub type DnsRecords = Vec<DnsRecord>;

impl NamedResponse for DnsRecord {
    fn name<'a>() -> Cow<'a, str> {
        "domain_record".into()
    }
}

impl fmt::Display for DnsRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "dns_record")
    }
}