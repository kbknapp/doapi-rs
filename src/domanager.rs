use crate::request::RequestBuilder;
use crate::response;
use reqwest::Method;
use url::Url;

#[derive(Debug, Default)]
pub struct DoManagerBuilder<'t> {
    auth: &'t str,
    endpoint_url: Option<Url>,
}

impl<'t> DoManagerBuilder<'t> {
    pub fn new() -> Self {
        DoManagerBuilder::default()
    }

    pub fn token(mut self, token: &'t str) -> Self {
        self.auth = token;
        self
    }

    pub fn endpoint_url(mut self, url: Url) -> Self {
        self.endpoint_url = Some(url);
        self
    }

    pub fn build(self) -> DoManager<'t> {
        self.into()
    }
}

/// The main structure through which all calls are made. This holds a slice of the AUTH TOKEN
#[derive(Clone, Debug)]
pub struct DoManager<'t> {
    pub(crate) auth: &'t str,
    pub(crate) endpoint_url: Url,
    pub(crate) client: reqwest::blocking::Client,
}

impl<'t> From<DoManagerBuilder<'t>> for DoManager<'t> {
    fn from(builder: DoManagerBuilder<'t>) -> Self {
        DoManager {
            auth: builder.auth,
            endpoint_url: if let Some(url) = builder.endpoint_url {
                url
            } else {
                Url::parse("https://api.digitalocean.com/v2/").unwrap()
            },
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl<'t> DoManager<'t> {
    pub fn builder() -> DoManagerBuilder<'t> {
        DoManagerBuilder::new()
    }
    /// Creates a new instance of `DoManager` with a string slice of your AUTH TOKEN
    pub fn with_token(token: &'t str) -> DoManager<'t> {
        DoManager::builder().token(token).build()
    }

    pub(crate) fn request_builder<T>(
        &self,
        method: Method,
        url: url::Url,
    ) -> RequestBuilder<'_, 't, T>
    where
        T: 't,
    {
        RequestBuilder {
            domgr: self,
            method,
            url,
            resp_t: std::marker::PhantomData,
            body: None,
        }
    }

    /// Returns a request that can be used to view account information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.account()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn account(&self) -> RequestBuilder<'_, 't, response::Account> {
        let url = self.endpoint_url.clone().join("account").unwrap();
        self.request_builder(Method::GET, url)
    }

    // /// Returns a request that can be used to list all regions
    // ///
    // /// # Example
    // ///
    // /// ```no_run
    // /// # use doapi::DoManager;
    // /// # use doapi::DoRequest;
    // /// let domgr = DoManager::with_token("asfasdfasdf");
    // /// match domgr.regions()
    // ///            .retrieve() {
    // ///     Ok(_)  => println!("Success"),
    // ///     Err(_) => println!("Error")
    // /// }
    // /// ```
    // pub fn regions(&self) -> RequestBuilder<'_, 't, response::Regions> {
    //     RequestBuilder::new(&self, "/regions")
    // }

    //     /// Returns a request that can be used to list all available sizes
    //     ///
    //     /// # Example
    //     ///
    //     /// ```no_run
    //     /// # use doapi::DoManager;
    //     /// # use doapi::DoRequest;
    //     /// let domgr = DoManager::with_token("asfasdfasdf");
    //     /// match domgr.sizes()
    //     ///            .retrieve() {
    //     ///     Ok(_)  => println!("Success"),
    //     ///     Err(_) => println!("Error")
    //     /// }
    //     /// ```
    //     pub fn sizes(&self) -> RequestBuilder<'t, response::Sizes> {
    //         RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/sizes")
    //     }

    /// Returns a request that can be used gain additional requests for a particular image
    ///
    /// **NOTE**: `id` may either be an image ID, or slug
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// // Or domgr.image("some slug")
    /// match domgr.image("1234")
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn image(&self, id: &str) -> RequestBuilder<'_, 't, response::Image> {
        let url = self
            .endpoint_url
            .clone()
            .join(&format!("images/{}", id))
            .unwrap();
        self.request_builder(Method::GET, url)
    }

    /// Returns a request that can be used to view all available images, or actions on multiple
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.images()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn images(&self) -> RequestBuilder<'_, 't, response::Images> {
        let url = self.endpoint_url.clone().join("images").unwrap();
        self.request_builder(Method::GET, url)
    }

    /// Returns a request that can be used to view all SSH keys or actions on multiple keys
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.ssh_keys()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn ssh_keys(&self) -> RequestBuilder<'_, 't, response::SshKeys> {
        let url = self.endpoint_url.clone().join("accounts/keys").unwrap();
        self.request_builder(Method::GET, url)
    }

    /// Returns a request that can be used to view a single SSH key, or actions that apply to only
    /// one key
    ///
    /// **NOTE**: `id` may either be an image ID, or slug
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// // or domgr.ssh_key("some finger print")
    /// match domgr.ssh_key("1234")
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn ssh_key(&self, id: &str) -> RequestBuilder<'_, 't, response::SshKey> {
        let url = self
            .endpoint_url
            .clone()
            .join(&format!("keys/{}", id))
            .unwrap();
        self.request_builder(Method::GET, url)
    }

    /// Returns a request that can be used to view a single droplet, or actions that only apply to
    /// one droplet
    ///
    /// **NOTE**: `id` may either be an image ID, or slug
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.droplet("1234")
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn droplet(&self, id: &str) -> RequestBuilder<'_, 't, response::Droplet> {
        let url = self
            .endpoint_url
            .clone()
            .join(&format!("droplets/{}", id))
            .unwrap();
        self.request_builder(Method::GET, url)
    }

    /// Returns a request that can be used to view all available droplets, or actions that apply to
    /// multiple droplets
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.droplets()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn droplets(&self) -> RequestBuilder<'_, 't, response::Droplets> {
        let url = self.endpoint_url.clone().join("droplets").unwrap();
        self.request_builder(Method::GET, url)
    }

    /// Returns a request that can be used to view all domains, or actions that apply to multiple
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.domains()
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn domains(&self) -> RequestBuilder<'_, 't, response::Domains> {
        let url = self.endpoint_url.clone().join("domains").unwrap();
        self.request_builder(Method::GET, url)
    }

    /// Returns a request that can be used to view a single domain, or actions that apply to only
    /// one
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// let domgr = DoManager::with_token("asfasdfasdf");
    /// match domgr.domain("super.com")
    ///            .retrieve() {
    ///     Ok(_)  => println!("Success"),
    ///     Err(_) => println!("Error")
    /// }
    /// ```
    pub fn domain(&self, name: &str) -> RequestBuilder<'_, 't, response::Domain> {
        let url = self
            .endpoint_url
            .clone()
            .join(&format!("domains/{}", name))
            .unwrap();
        self.request_builder(Method::GET, url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use httpmock::Method::GET;
    use httpmock::MockServer;

    use crate::tests::{BEARER_HEADER, TEST_TOKEN};
    use crate::DoRequest;

    #[test]
    fn test_do_manager() {
        let url = Url::parse("https://api.digitalocean.com/v2/").unwrap();
        let domgr = DoManager::with_token(TEST_TOKEN);

        assert_eq!(domgr.endpoint_url, url);
        assert_eq!(domgr.auth, TEST_TOKEN);
    }

    #[test]
    fn test_account() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/account")
                .header("Authorization", &BEARER_HEADER);

            then.status(200)
                .header("Content-Type", "application/json; charset=utf-8")
                .header("ratelimit-limit", "1200")
                .header("ratelimit-remaining", "1137")
                .header("ratelimit-reset", "1415984218")
                .body(
                    r#"{
                    "account": {
                      "droplet_limit": 25,
                      "floating_ip_limit": 5,
                      "volume_limit": 10,
                      "email": "sammy@digitalocean.com",
                      "uuid": "b6fr89dbf6d9156cace5f3c78dc9851d957381ef",
                      "email_verified": true,
                      "status": "active",
                      "status_message": ""
                    }
                  }"#,
                );
        });

        let url = Url::parse(&server.url("/account")).unwrap();

        let domgr = DoManager::builder()
            .token(TEST_TOKEN)
            .endpoint_url(url)
            .build();

        let account = domgr.account().retrieve().unwrap();

        assert_eq!(account.droplet_limit as u32, 25);
        assert_eq!(account.floating_ip_limit as u32, 5);
        assert_eq!(account.volume_limit as u32, 10);
        assert_eq!(account.email, "sammy@digitalocean.com");
        assert_eq!(account.uuid, "b6fr89dbf6d9156cace5f3c78dc9851d957381ef");
        assert_eq!(account.email_verified, true);
        assert_eq!(account.status, "active");
        assert_eq!(account.status_message, "");

        mock.assert();
    }
}
