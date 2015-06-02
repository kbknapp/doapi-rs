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
        //      "priority" : "20"       // MX, SRV
        //      "port" : "80"           // SRV
        //      "weight" : "200"        // SRV

        // FIXME: Don't unwrap()
        RequestBuilder {
            method: Method::Post,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(json::to_string(rec).ok().unwrap())
        }


        // let mut hm = HashMap::new();
        // hm.insert("type", rec.rec_type.to_string());
        // if let Some(ref n) = rec.name {
        //     hm.insert("name", n.to_owned());
        // }
        // if let Some(ref d) = rec.data {
        //     hm.insert("data", d.to_owned());
        // }
        // if let Some(p) = rec.priority {
        //     hm.insert("priority", p.to_string());
        // }
        // if let Some(p) = rec.port {
        //     hm.insert("name", p.to_string());
        // }
        // if let Some(w) = rec.weight {
        //     hm.insert("name", w.to_string());
        // }
    }
}

impl<'t> RequestBuilder<'t, response::DnsRecord> {
    pub fn update(mut self, record: &DnsRecord) -> RequestBuilder<'t, response::DnsRecord> {
        // PUT: "https://api.digitalocean.com/v2/domains/$DOMAIN/records/$ID"
        // body:
        //      "type" : "MX"           // All records
        //      "name" : "alias"        // A, AAAA, CNAME, TXT, SRV
        //      "data" : "varies"       // A, AAAA, CNAME, MX, TXT, SRV, NS
        //      "priority" : "20"       // MX, SRV
        //      "port" : "80"           // SRV
        //      "weight" : "200"        // SRV
        // FIXME: Don't unwrap()
        RequestBuilder {
            method: Method::Put,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(json::to_string(record).ok().unwrap())
        }

        // let mut hm = HashMap::new();
        // hm.insert("type", record.rec_type.to_string());
        // if let Some(ref n) = record.name {
        //     hm.insert("name", n.to_owned());
        // }
        // if let Some(ref d) = record.data {
        //     hm.insert("data", d.to_owned());
        // }
        // if let Some(p) = record.priority {
        //     hm.insert("priority", p.to_string());
        // }
        // if let Some(p) = record.port {
        //     hm.insert("name", p.to_string());
        // }
        // if let Some(w) = record.weight {
        //     hm.insert("name", w.to_string());
        // }
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
