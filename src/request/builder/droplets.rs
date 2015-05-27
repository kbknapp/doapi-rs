use std::fmt;

use response;
use request::RequestBuilder;
use request::DoRequest;

pub struct Droplet {
    pub name: String,
    pub region: String,
    pub size: String,
    pub image: String,
    pub ssh_keys: Option<Vec<String>>,
    pub backups: bool,
    pub ipv6: bool,
    pub private_net: bool,
    pub data: Option<String>
}

impl fmt::Display for Droplet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "Name: {}\n\
             Region: {}\n\
             Size: {}\n\
             Image: {}\n\
             SSH Keys: {}\n\
             Backups Enabled: {}\n\
             IPv6 Enabled: {}\n\
             Private Networking Enabled: {}\n\
             User Data: {}\n",
             self.name,
             self.region,
             self.size,
             self.image,
             if let Some(ref v) = self.ssh_keys {
                v.iter().fold(String::new(), |acc, s| acc + &format!(" {},", s)[..])
             } else {
                "None".to_owned()
             },
             self.backups,
             self.ipv6,
             self.private_net,
             if let Some(d) = self.data.clone() {
                d
             } else {
                "None".to_owned()
             }
        )
    }
}

impl<'t> RequestBuilder<'t, response::Droplet> {
    pub fn update(&self, id: &str) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn disable_backups(&self) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn reboot(&self) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn power_cycle(&self) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn shutdown(&self) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn power_off(&self) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn power_on(&self) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn restore(&self, img: &str) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn reset_password(&self) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn resize(&self, size: &str, disk: bool) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn rebuild(&self, img: &str) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn rename(&self, name: &str) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn change_kernel(&self, kernel: &str) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn enable_ipv6(&self) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn enable_private_networking(&self) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn snapshot(&self) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn upgrade(&self) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn action(&self, id: &str) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn kernels(&self) -> RequestBuilder<'t, response::Kernels> {
        unimplemented!()
    }
    pub fn snapshots(&self) -> RequestBuilder<'t, response::Snapshots> {
        unimplemented!()
    }
    pub fn backups(&self) -> RequestBuilder<'t, response::Backups> {
        unimplemented!()
    }
    pub fn actions(&self) -> RequestBuilder<'t, response::Actions> {
        unimplemented!()
    }
    pub fn delete(&self) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn show(&self) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn neighbors(&self) -> RequestBuilder<'t, response::Droplets> {
        unimplemented!()
    }
}

impl<'t> RequestBuilder<'t, response::Droplets> {
    pub fn create(&self, droplet: &Droplet) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn neighbors(&self) -> RequestBuilder<'t, response::Droplets> {
        unimplemented!()
    }
    pub fn all_neighbors(&self) -> RequestBuilder<'t, response::Droplets> {
        unimplemented!()
    }
    pub fn upgrades(&self) -> RequestBuilder<'t, response::Droplets> {
        unimplemented!()
    }
}

impl<'t> DoRequest<response::Droplet> for RequestBuilder<'t, response::Droplet> {}

