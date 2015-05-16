
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    droplet_limit: i32,
    email: String,
    uuid: String,
    email_verified: bool
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DigitalOcean Account:\n\t\
                        Email: {}\n\t\
                        Droplet Limit: {}\n\t\
                        UUID: {}\n\t\
                        E-Mail Verified: {}",
                self.email,
                self.droplet_limit,
                self.uuid,
                self.email_verified)
    }
}
