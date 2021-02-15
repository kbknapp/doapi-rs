//use std::marker::PhantomData;

use crate::request::{DoRequest, RequestBuilder};
use crate::response;

// impl<'t> RequestBuilder<'t, response::Domains> {
//     /// Returns a `RequestBuilder` that can be used to create a new domain.
//     ///
//     /// # Example
//     ///
//     /// ```no_run
//     /// # use doapi::DoManager;
//     /// # use doapi::DoRequest;
//     /// # let domgr = DoManager::with_token("asfasdfasdf");
//     /// // ... domgr set up same as before
//     /// match domgr.domains()
//     ///            .create("super.com", "10.10.10.1")
//     ///            .retrieve() {
//     ///     Ok(domain) => println!("Domain: {}", domain),
//     ///     Err(e)     => println!("Error: {}", e)
//     /// }
//     /// ```
//     pub fn create(self, name: &str, ip: &str) -> RequestBuilder<'t, response::Domain> {
//         // POST: "https://api.digitalocean.com/v2/domains"
//         // body:
//         //      "ip_address" : "192.168.1.1"
//         //      "name" : "supercool.com"
//         RequestBuilder {
//             method: Method::Post,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: Some(format!("{{\"name\":{:?},\"ip_address\":{:?}}}", name, ip)),
//         }
//     }
// }

// impl<'t> RequestBuilder<'t, response::Domain> {
//     /// Returns a `RequestBuilder` that can be used delete an existing domain.
//     ///
//     /// # Example
//     ///
//     /// ```no_run
//     /// # use doapi::DoManager;
//     /// # use doapi::DoRequest;
//     /// # let domgr = DoManager::with_token("asfasdfasdf");
//     /// // ... domgr set up same as before
//     /// match domgr.domain("super.com")
//     ///            .delete()
//     ///            .retrieve() {
//     ///     Ok(_)  => println!("Success"),
//     ///     Err(_) => println!("Error")
//     /// }
//     /// ```
//     pub fn delete(self) -> RequestBuilder<'t, response::HeaderOnly> {
//         // DELETE: "https://api.digitalocean.com/v2/domains/$id"
//         RequestBuilder {
//             method: Method::Delete,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: None,
//         }
//     }

//     /// Returns a type of `RequestBuilder` which allows you make requests related to multiple DNS
//     /// records or the concept of "DNS Records" as a whole
//     ///
//     /// # Example
//     ///
//     /// ```no_run
//     /// # use doapi::DoManager;
//     /// # use doapi::DoRequest;
//     /// # let domgr = DoManager::with_token("asfasdfasdf");
//     /// // ... domgr set up same as before
//     /// match domgr.domains()
//     ///            .create("super.com", "10.10.10.1")
//     ///            .retrieve() {
//     ///     Ok(domain) => println!("Domain: {}", domain),
//     ///     Err(e)     => println!("Error: {}", e)
//     /// }
//     /// ```
//     pub fn dns_records(mut self) -> RequestBuilder<'t, response::DnsRecords> {
//         // GET: "https://api.digitalocean.com/v2/domains/$DOMAIN/records"
//         self.url.push('/');
//         self.url.push_str("records");
//         RequestBuilder::new(self.auth, self.url)
//     }

//     /// Returns type of `RequestBuilder` which allows you make requests related to a single DNS
//     /// record
//     ///
//     /// # Example
//     ///
//     /// ```no_run
//     /// # use doapi::DoManager;
//     /// # use doapi::DoRequest;
//     /// # let domgr = DoManager::with_token("asfasdfasdf");
//     /// // ... domgr set up same as before
//     /// match domgr.domain("super.com")
//     ///            .retrieve() {
//     ///     Ok(domain) => println!("Domain: {}", domain),
//     ///     Err(e)     => println!("Error: {}", e)
//     /// }
//     /// ```
//     pub fn dns_record(mut self, id: &str) -> RequestBuilder<'t, response::DnsRecord> {
//         // GET "https://api.digitalocean.com/v2/domains/$DOMAIN/records/$ID"
//         self.url.push('/');
//         self.url.push_str(id);
//         RequestBuilder::new(self.auth, self.url)
//     }
// }

impl<'a, 't> DoRequest<response::Domain> for RequestBuilder<'a, 't, response::Domain> {}
