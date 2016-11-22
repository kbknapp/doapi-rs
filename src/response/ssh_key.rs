// id               number      This is a unique identification number for the
// key. This can be used to reference a specific SSH key when you wish to embed
// a key into a Droplet.
// fingerprint      string      This attribute contains the fingerprint value
// that is generated from the public key. This is a unique identifier that will
// differentiate it from other keys using a format that SSH recognizes.
// public_key       string      This attribute contains the entire public key
// string that was uploaded. This is what is embedded into the root user's
// authorized_keys file if you choose to include this SSH key during Droplet
// creation.
// name             string      This is the human-readable display name for the
// given SSH key. This is used to easily identify the SSH keys when they are
// displayed.

use std::fmt;
use std::borrow::Cow;

use response::NamedResponse;
use response;

#[derive(Deserialize, Debug)]
pub struct SshKey {
    pub id: f64,
    pub fingerprint: String,
    pub public_key: String,
    pub name: String,
}

impl response::NotArray for SshKey {}

impl NamedResponse for SshKey {
    fn name<'a>() -> Cow<'a, str> { "ssh_key".into() }
}

impl fmt::Display for SshKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "ID: {:.0}\n\
                  Fingerprint: {}\n\
                  Public Key: {}\n\
                  Name: {}",
               self.id,
               self.fingerprint,
               self.public_key,
               self.name)
    }
}

pub type SshKeys = Vec<SshKey>;
