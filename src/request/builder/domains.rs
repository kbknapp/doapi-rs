
use response;
use request::RequestBuilder;
use request::DoRequest;

pub struct DomainRecord;

impl<'t> RequestBuilder<'t, response::Domains> {
    pub fn create(&self, name: &str, ip: &str) -> RequestBuilder<'t, response::Domain> {
        unimplemented!()
    }
    pub fn show(&self, id: &str) -> RequestBuilder<'t, response::Domain> {
        unimplemented!()
    }
    pub fn delete(&self, id: &str) -> RequestBuilder<'t, response::Domain> {
        unimplemented!()
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
    pub fn delete(&self, id: &str) -> RequestBuilder<'t, response::DnsRecord> {
        unimplemented!()
    }
    pub fn show(&self, id: &str) -> RequestBuilder<'t, response::DnsRecord> {
        unimplemented!()
    }
}

impl<'t> DoRequest<response::Domain> for RequestBuilder<'t, response::Domain> {}