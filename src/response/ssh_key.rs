// id               number      This is a unique identification number for the key. This can be used to reference a specific SSH key when you wish to embed a key into a Droplet.
// fingerprint      string      This attribute contains the fingerprint value that is generated from the public key. This is a unique identifier that will differentiate it from other keys using a format that SSH recognizes.
// public_key       string      This attribute contains the entire public key string that was uploaded. This is what is embedded into the root user's authorized_keys file if you choose to include this SSH key during Droplet creation.
// name             string      This is the human-readable display name for the given SSH key. This is used to easily identify the SSH keys when they are displayed.

use std::fmt;

#[derive(Deserialize, Debug)]
pub struct SshKey {
    id: f64,
    fingerprint: String,
    public_key: String,
    name: String,
}

impl fmt::Display for SshKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "SSH Key:\n\t\
                        ID: {:.0}\n\t\
                        Fingerprint: {}\n\t\
                        Public Key:{}\n\t\
                        Name:{}\n",
                self.id,
                self.fingerprint,
                self.public_key,
                self.name)
    }
}