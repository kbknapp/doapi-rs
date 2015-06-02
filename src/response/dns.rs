// id           number              A unique identifier for each domain record.
// type         string              The type of the DNS record (ex: A, CNAME, TXT, ...).
// name         string              The name to use for the DNS record.
// data         string              The value to use for the DNS record.
// priority     nullable number     The priority for SRV and MX records.
// port         nullable number     The port for SRV records.
// weight       nullable number     The weight for SRV records.

use std::borrow::Cow;
use std::fmt;

use response::NamedResponse;

#[derive(Deserialize)]
pub struct DnsRecord {
    pub id: f64,
    #[serde(rename="type")]
    pub rec_type: String,
    pub name: String,
    pub data: String,
    pub priority: Option<f64>,
    pub port: Option<f64>,
    pub weight: Option<f64>,
}

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