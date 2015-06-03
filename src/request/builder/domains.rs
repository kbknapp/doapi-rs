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
        RequestBuilder {
            method: Method::Delete,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(format!("{{\"name\":{:?},\"ip_address\":{:?}}}", name, ip)) 
        }
    }
}

impl<'t> RequestBuilder<'t, response::Domain> {
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
    pub fn dns_records(mut self) -> RequestBuilder<'t, response::DnsRecords> {
        // GET: "https://api.digitalocean.com/v2/domains/$DOMAIN/records"
        self.url.push('/');
        self.url.push_str("records");
        RequestBuilder::new(self.auth, self.url)
    }
    pub fn dns_record(mut self, id: &str) -> RequestBuilder<'t, response::DnsRecord> {
        // GET "https://api.digitalocean.com/v2/domains/$DOMAIN/records/$ID"
        self.url.push('/');
        self.url.push_str(id);
        RequestBuilder::new(self.auth, self.url)
    }
}

impl<'t> DoRequest<response::Domain> for RequestBuilder<'t, response::Domain> {}