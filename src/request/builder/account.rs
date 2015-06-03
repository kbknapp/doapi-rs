use response;
use request::RequestBuilder;
use request::DoRequest;

impl<'t> RequestBuilder<'t, response::Account> {
    /// Returns a struct for making requests that send back "Action" objects. The `id` is the
    /// action ID you'd like to retrieve from DigitalOcean
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # let domgr = DoManager::with_auth("asfasdfasdf");
    /// // ... domgr set up same as before
    /// match domgr.account().action("1234").retrieve() {
    ///     Ok(action) => println!("Action: {}", action),
    ///     Err(e)     => println!("Error: {}", e)
    /// }
    /// ```
    pub fn action(self, id: &str) -> RequestBuilder<'t, response::Action> {
        // https://api.digitalocean.com/v2/actions/$ID
        RequestBuilder::new(self.auth, format!("https://api.digitalocean.com/v2/actions/{}", id))
    }

    /// Returns a `Vec<Action>` for making requests that send back multiple "Action" objects. 
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # let domgr = DoManager::with_auth("asfasdfasdf");
    /// // ... domgr set up same as before
    /// match domgr.account().action("1234").retrieve() {
    ///     Ok(action) => println!("Action: {}", action),
    ///     Err(e)     => println!("Error: {}", e)
    /// }
    /// ```
    pub fn actions(self) -> RequestBuilder<'t, response::Actions> {
        // https://api.digitalocean.com/v2/actions
        RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/actions")
    }
}

impl<'t> DoRequest<response::Account> for RequestBuilder<'t, response::Account> {}
