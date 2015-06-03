use std::fmt;
use std::marker::PhantomData;

use hyper::method::Method;
use serde::json;

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

// type     string  The record type (A, MX, CNAME, etc).    All Records
// name     string  The host name, alias, or service being defined by the record.   A, AAAA, CNAME, TXT, SRV
// data     string  Variable data depending on record type. See the [Domain Records]() section for more detail on each record type. A, AAAA, CNAME, MX, TXT, SRV, NS
// priority    nullable number The priority of the host (for SRV and MX records. null otherwise).  MX, SRV
// port     nullable number The port that the service is accessible on (for SRV records only. null otherwise).  SRV
// weight   nullable number The weight of records with the same priority (for SRV records only. null otherwise).    SRV
#[derive(Serialize)]
pub struct DnsRecord {
    #[serde(rename = "type")]
    pub rec_type: String,
    pub name: Option<String>,
    pub priority: Option<u64>,
    pub port: Option<u64>,
    pub data: Option<String>,
    pub weight: Option<u64>,
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

impl<'t> RequestBuilder<'t, response::DnsRecords> {
    pub fn create(self, rec: &DnsRecord) -> RequestBuilder<'t, response::DnsRecord> {
        // POST: "https://api.digitalocean.com/v2/domains/$DOMAIN/records"
        // body:
        //      "type" : "MX"           // All records
        //      "name" : "alias"        // A, AAAA, CNAME, TXT, SRV
        //      "data" : "varies"       // A, AAAA, CNAME, MX, TXT, SRV, NS
        //      "priority" : 20       // MX, SRV
        //      "port" : 80           // SRV
        //      "weight" : 200        // SRV

        // FIXME: Don't unwrap()
        RequestBuilder {
            method: Method::Post,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(json::to_string(rec).ok().unwrap())
        }
    }
}

impl<'t> RequestBuilder<'t, response::DnsRecord> {
    pub fn update(self, record: &DnsRecord) -> RequestBuilder<'t, response::DnsRecord> {
        // PUT: "https://api.digitalocean.com/v2/domains/$DOMAIN/records/$ID"
        // body:
        //      "type" : "MX"           // All records
        //      "name" : "alias"        // A, AAAA, CNAME, TXT, SRV
        //      "data" : "varies"       // A, AAAA, CNAME, MX, TXT, SRV, NS
        //      "priority" : 20       // MX, SRV
        //      "port" : 80           // SRV
        //      "weight" : 200        // SRV
        // FIXME: Don't unwrap()
        RequestBuilder {
            method: Method::Put,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(json::to_string(record).ok().unwrap())
        }
    }
    pub fn delete(self) -> RequestBuilder<'t, response::HeaderOnly> {
        // DELETE: "https://api.digitalocean.com/v2/domains/$id"
        RequestBuilder {
            method: Method::Delete,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None
        }
    }
}

impl<'t> DoRequest<response::DnsRecord> for RequestBuilder<'t, response::DnsRecord> {}
