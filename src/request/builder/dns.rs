use std::fmt;

use response;
use request::RequestBuilder;
use request::DoRequest;

doapi_enum! {
    #[derive(Debug)]
    pub enum DnsRecType {
        A,
        AAAA,
        CNAME,
        MX,
        NS,
        SRV,
        TXT
    }
}

pub struct DnsRecord {
    pub rec_type: DnsRecType,
    pub name: Option<String>,
    pub priority: Option<u32>,
    pub port: Option<u32>,
    pub data: Option<String>,
    pub weight: Option<u32>,
}

impl fmt::Display for DnsRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "Record Type: {}\n\
             Name: {}\n\
             Data: {}\n\
             Priority: {}\n\
             Port: {}\n\
             Weight: {}\n",
             self.rec_type,
             if let Some(n) = self.name.clone() {
                n
             } else {
                "None".to_owned()
             },
             if let Some(d) = self.data.clone() {
                d
             } else {
                "None".to_owned()
             },
             if let Some(p) = self.priority {
                p.to_string()
             } else {
                "None".to_owned()
             },
             if let Some(p) = self.port {
                p.to_string()
             } else {
                "None".to_owned()
             },
             if let Some(w) = self.weight {
                w.to_string()
             } else {
                "None".to_owned()
             }
        )
    }
}
impl<'t> RequestBuilder<'t, response::DnsRecord> {
    pub fn create(&self, rec: &DnsRecord) -> RequestBuilder<'t, response::DnsRecord> {
        unimplemented!()
    }
    pub fn records(&self) -> RequestBuilder<'t, response::DnsRecords> {
        unimplemented!()
    }
    pub fn update(&self, id: &str, record: &DnsRecord) -> RequestBuilder<'t, response::DnsRecord> {
        unimplemented!()
    }
    pub fn delete(&self, id: &str) -> RequestBuilder<'t, response::DnsRecord> {
        unimplemented!()
    }
    pub fn show(&self, id: &str) -> RequestBuilder<'t, response::DnsRecord> {
        unimplemented!()
    }
}

impl<'t> DoRequest<response::DnsRecord> for RequestBuilder<'t, response::DnsRecord> {}
