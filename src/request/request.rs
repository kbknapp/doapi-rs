pub struct Request<'u, 't> {
    url: &'u str,
    auth: &'t str
}

impl<'u, 't> Request<'u, 't> {
    pub fn new(url: &'u str, token: &'t str) -> Request {
        Request {
            url: url,
            auth: token
        }
    }

    pub fn request(&self) -> hyper::Result<Request<Fresh>> {
        let url = match Url::parse(self.url) {
            Ok(url) => url,
            Err(e) => return Err(Error::Uri(e)),
        };
        let mut fresh_req = match Request::new(Method::Get, url) {
            Ok(req) => req,
            Err(e)  => return Err(e)
        };
        let mut auth_s = String::new();
        auth_s.push_str("Bearer ");
        auth_s.push_str(self.auth);
        fresh_req.headers_mut().set(ContentType("application/json".parse().unwrap()));
        fresh_req.headers_mut().set(Authorization(auth_s));
        Ok(fresh_req)
    }

    pub fn retrieve_json(&self) -> hyper::Result<String> {
        let url = match Url::parse(self.url) {
            Ok(url) => url,
            Err(e) => return Err(Error::Uri(e)),
        };
        let mut fresh_req = match Request::new(Method::Get, url) {
            Ok(req) => req,
            Err(e)  => return Err(e)
        };
        let mut auth_s = String::new();
        auth_s.push_str("Bearer ");
        auth_s.push_str(self.auth);
        fresh_req.headers_mut().set(ContentType("application/json".parse().unwrap()));
        fresh_req.headers_mut().set(Authorization(auth_s));
        let streaming_req = try!(fresh_req.start());
        let mut response = try!(streaming_req.send());
        let mut s = String::new();
        try!(response.read_to_string(&mut s));
        Ok(s)
    }

    pub fn retrieve<T>(&self, obj: &str) -> Result<T, String> {
        match self.retrieve_json() {
            Ok(ref s) => {
                match json::from_str::<Value>(s) {
                    Ok(obj) => {
                        match obj.find(obj) {
                            Some(a) => {
                                match json::from_value(a.clone()) {
                                    Ok(a) => Ok(a),
                                    Err(e) => Err(e.into())
                                }
                            },
                            None => Err(json::Error::MissingFieldError(obj).into())
                        }
                    },
                    Err(e) => Err(e.into())
                }
            },
            Err(e) => Err(e.into())
        }
    }
}
