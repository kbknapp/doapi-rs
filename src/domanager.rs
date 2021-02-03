use url::Url;
use crate::response;
//use request::RequestBuilder;

/// The main structure through which all calls are made. This holds a slice of the AUTH TOKEN
#[derive(Clone, Debug)]
pub struct DoManager<'t> {
    auth: &'t str,
    endpoint_url: Url,
    client: reqwest::blocking::Client,
}

impl<'t> DoManager<'t> {
    /// Creates a new instance of `DoManager` with a string slice of your AUTH TOKEN
    pub fn with_token(token: &'t str) -> DoManager<'t> {
        DoManager {
            auth: token,
            endpoint_url: Url::parse("https://api.digitalocean.com/v2/").unwrap(),
            client: reqwest::blocking::Client::new(),
        }
    }

    //     /// Returns a request that can be used to view account information.
    //     ///
    //     /// # Example
    //     ///
    //     /// ```no_run
    //     /// # use doapi::DoManager;
    //     /// # use doapi::DoRequest;
    //     /// let domgr = DoManager::with_token("asfasdfasdf");
    //     /// match domgr.account()
    //     ///            .retrieve() {
    //     ///     Ok(_)  => println!("Success"),
    //     ///     Err(_) => println!("Error")
    //     /// }
    //     /// ```
    //     pub fn account(&self) -> RequestBuilder<'t, response::Account> {
    //         RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/account")
    //     }

    pub fn set_endpoint_url(&mut self, url: Url) {
        self.endpoint_url = url;
    }

    pub fn endpoint_url(&self) -> &Url {
        &self.endpoint_url
    }

    //     /// Returns a request that can be used to list all regions
    //     ///
    //     /// # Example
    //     ///
    //     /// ```no_run
    //     /// # use doapi::DoManager;
    //     /// # use doapi::DoRequest;
    //     /// let domgr = DoManager::with_token("asfasdfasdf");
    //     /// match domgr.regions()
    //     ///            .retrieve() {
    //     ///     Ok(_)  => println!("Success"),
    //     ///     Err(_) => println!("Error")
    //     /// }
    //     /// ```
    //     pub fn regions(&self) -> RequestBuilder<'t, response::Regions> {
    //         RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/regions")
    //     }

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

    //     /// Returns a request that can be used gain additional requests for a particular image
    //     ///
    //     /// **NOTE**: `id` may either be an image ID, or slug
    //     ///
    //     /// # Example
    //     ///
    //     /// ```no_run
    //     /// # use doapi::DoManager;
    //     /// # use doapi::DoRequest;
    //     /// let domgr = DoManager::with_token("asfasdfasdf");
    //     /// // Or domgr.image("some slug")
    //     /// match domgr.image("1234")
    //     ///            .retrieve() {
    //     ///     Ok(_)  => println!("Success"),
    //     ///     Err(_) => println!("Error")
    //     /// }
    //     /// ```
    //     pub fn image(&self, id: &str) -> RequestBuilder<'t, response::Image> {
    //         RequestBuilder::new(self.auth,
    //                             format!("https://api.digitalocean.com/v2/images/{}", id))
    //     }

    //     /// Returns a request that can be used to view all available images, or actions on multiple
    //     ///
    //     /// # Example
    //     ///
    //     /// ```no_run
    //     /// # use doapi::DoManager;
    //     /// # use doapi::DoRequest;
    //     /// let domgr = DoManager::with_token("asfasdfasdf");
    //     /// match domgr.images()
    //     ///            .retrieve() {
    //     ///     Ok(_)  => println!("Success"),
    //     ///     Err(_) => println!("Error")
    //     /// }
    //     /// ```
    //     pub fn images(&self) -> RequestBuilder<'t, response::Images> {
    //         RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/images")
    //     }

    //     /// Returns a request that can be used to view all SSH keys or actions on multiple keys
    //     ///
    //     /// # Example
    //     ///
    //     /// ```no_run
    //     /// # use doapi::DoManager;
    //     /// # use doapi::DoRequest;
    //     /// let domgr = DoManager::with_token("asfasdfasdf");
    //     /// match domgr.ssh_keys()
    //     ///            .retrieve() {
    //     ///     Ok(_)  => println!("Success"),
    //     ///     Err(_) => println!("Error")
    //     /// }
    //     /// ```
    //     pub fn ssh_keys(&self) -> RequestBuilder<'t, response::SshKeys> {
    //         RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/account/keys")
    //     }

    //     /// Returns a request that can be used to view a single SSH key, or actions that apply to only
    //     /// one key
    //     ///
    //     /// **NOTE**: `id` may either be an image ID, or slug
    //     ///
    //     /// # Example
    //     ///
    //     /// ```no_run
    //     /// # use doapi::DoManager;
    //     /// # use doapi::DoRequest;
    //     /// let domgr = DoManager::with_token("asfasdfasdf");
    //     /// // or domgr.ssh_key("some finger print")
    //     /// match domgr.ssh_key("1234")
    //     ///            .retrieve() {
    //     ///     Ok(_)  => println!("Success"),
    //     ///     Err(_) => println!("Error")
    //     /// }
    //     /// ```
    //     pub fn ssh_key(&self, id: &str) -> RequestBuilder<'t, response::SshKey> {
    //         RequestBuilder::new(self.auth,
    //                             format!("https://api.digitalocean.com/v2/account/keys/{}", id))
    //     }

    //     /// Returns a request that can be used to view a single droplet, or actions that only apply to
    //     /// one droplet
    //     ///
    //     /// **NOTE**: `id` may either be an image ID, or slug
    //     ///
    //     /// # Example
    //     ///
    //     /// ```no_run
    //     /// # use doapi::DoManager;
    //     /// # use doapi::DoRequest;
    //     /// let domgr = DoManager::with_token("asfasdfasdf");
    //     /// match domgr.droplet("1234")
    //     ///            .retrieve() {
    //     ///     Ok(_)  => println!("Success"),
    //     ///     Err(_) => println!("Error")
    //     /// }
    //     /// ```
    //     pub fn droplet(&self, id: &str) -> RequestBuilder<'t, response::Droplet> {
    //         RequestBuilder::new(self.auth,
    //                             format!("https://api.digitalocean.com/v2/droplets/{}", id))
    //     }

    //     /// Returns a request that can be used to view all available droplets, or actions that apply to
    //     /// multiple droplets
    //     ///
    //     /// # Example
    //     ///
    //     /// ```no_run
    //     /// # use doapi::DoManager;
    //     /// # use doapi::DoRequest;
    //     /// let domgr = DoManager::with_token("asfasdfasdf");
    //     /// match domgr.droplets()
    //     ///            .retrieve() {
    //     ///     Ok(_)  => println!("Success"),
    //     ///     Err(_) => println!("Error")
    //     /// }
    //     /// ```
    //     pub fn droplets(&self) -> RequestBuilder<'t, response::Droplets> {
    //         RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/droplets")
    //     }

    //     /// Returns a request that can be used to view all domains, or actions that apply to multiple
    //     ///
    //     /// # Example
    //     ///
    //     /// ```no_run
    //     /// # use doapi::DoManager;
    //     /// # use doapi::DoRequest;
    //     /// let domgr = DoManager::with_token("asfasdfasdf");
    //     /// match domgr.domains()
    //     ///            .retrieve() {
    //     ///     Ok(_)  => println!("Success"),
    //     ///     Err(_) => println!("Error")
    //     /// }
    //     /// ```
    //     pub fn domains(&self) -> RequestBuilder<'t, response::Domains> {
    //         RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/domains")
    //     }

    //     /// Returns a request that can be used to view a single domain, or actions that apply to only
    //     /// one
    //     ///
    //     /// # Example
    //     ///
    //     /// ```no_run
    //     /// # use doapi::DoManager;
    //     /// # use doapi::DoRequest;
    //     /// let domgr = DoManager::with_token("asfasdfasdf");
    //     /// match domgr.domain("super.com")
    //     ///            .retrieve() {
    //     ///     Ok(_)  => println!("Success"),
    //     ///     Err(_) => println!("Error")
    //     /// }
    //     /// ```
    //     pub fn domain(&self, name: &str) -> RequestBuilder<'t, response::Domain> {
    //         RequestBuilder::new(self.auth,
    //                             format!("https://api.digitalocean.com/v2/domains/{}", name))
    //     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_manager() {
        let token = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let url = Url::parse("https://api.digitalocean.com/v2/").unwrap();
        let mut domgr = DoManager::with_token(&token);

        assert_eq!(*domgr.endpoint_url(), url);
        assert_eq!(domgr.auth, token);
        
        let test_url = Url::parse("http://localhost").unwrap();
        domgr.set_endpoint_url(test_url.clone());

        assert_eq!(*domgr.endpoint_url(), test_url);
        

    }
}
