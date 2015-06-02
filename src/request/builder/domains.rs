use std::collections::HashMap;
use std::marker::PhantomData;

use hyper::method::Method;

use response;
use request::RequestBuilder;
use request::DoRequest;

pub struct DomainRecord;

impl<'t> RequestBuilder<'t, response::Domains> {
    pub fn create(self, name: &str, ip: &str) -> RequestBuilder<'t, response::Domain> {
        // POST: "https://api.digitalocean.com/v2/domains"
        // body:
        //      "ip_address" : "192.168.1.1"
        //      "name" : "supercool.com"
        let mut hm = HashMap::new();
        hm.insert("name", name.to_owned());
        hm.insert("ip_address", ip.to_owned());
        RequestBuilder {
            method: Method::Delete,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(hm) 
        }
    }
}

impl<'t> RequestBuilder<'t, response::Domain> {
    pub fn create(&self, rec: &DomainRecord) -> RequestBuilder<'t, response::DnsRecord> {
        unimplemented!()
    }
    pub fn records(&self) -> RequestBuilder<'t, response::DnsRecords> {
        unimplemented!()
    }
    pub fn update(&self, id: &str) -> RequestBuilder<'t, response::DnsRecord> {
        unimplemented!()
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
    pub fn show(self) -> RequestBuilder<'t, response::Domain> {
        // GET: "https://api.digitalocean.com/v2/domains/$ID"
        self
    }
}

impl<'t> DoRequest<response::Domain> for RequestBuilder<'t, response::Domain> {}