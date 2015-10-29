use response;
use request::RequestBuilder;
use request::DoRequest;


impl<'t> RequestBuilder<'t, response::Account> {
    /// Returns type of `RequestBuilder` which allows you make requests for information related to 
    /// a single action
    ///
    /// **Parameters:**
    /// `id`: The action ID you'd like to retrieve from DigitalOcean
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// # let domgr = DoManager::with_token("asfasdfasdf");
    /// // ... domgr set up same as before
    /// match domgr.account().action("1234").retrieve() {
    ///     Ok(action) => println!("Action: {}", action),
    ///     Err(e)     => println!("Error: {}", e)
    /// }
    /// ```
    pub fn action(self, id: &str) -> RequestBuilder<'t, response::Action> {
        // https://api.digitalocean.com/v2/actions/$ID
        RequestBuilder::new(self.auth,
                            format!("https://api.digitalocean.com/v2/actions/{}", id))
    }

    /// A type of `RequestBuilder` that lets you make requests for multiple actions or the concept 
    /// of "Actions" as a whole
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// # let domgr = DoManager::with_token("asfasdfasdf");
    /// // ... domgr set up same as before
    /// match domgr.account().actions().retrieve() {
    ///     Ok(action_vec) => println!("Action: {:?}", action_vec),
    ///     Err(e)     => println!("Error: {}", e)
    /// }
    /// ```
    pub fn actions(self) -> RequestBuilder<'t, response::Actions> {
        // https://api.digitalocean.com/v2/actions
        RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/actions")
    }
}

impl<'t> DoRequest<response::Account> for RequestBuilder<'t, response::Account> {}
