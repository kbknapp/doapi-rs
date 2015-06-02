use std::fmt;
use std::marker::PhantomData;

use hyper::method::Method;
use serde::json;

use response;
use request::RequestBuilder;
use request::DoRequest;

// name (true)  String           The human-readable string you wish to use when displaying the 
//                               Droplet name. The name, if set to a domain name managed in the 
//                               DigitalOcean DNS management system, will configure a PTR record 
//                               for the Droplet. The name set during creation will also determine 
//                               the hostname for the Droplet in its internal configuration.
// region (true) String          The unique slug id for the region that you wish to deploy in
// size (true)   String          The unique slug identifier for the size that you wish to select 
//                               for this Droplet.
// image         number (if using an image ID), or String (if using a public image slug) The image ID of a public or private image, or the unique slug identifier for a public image. This image will be the base image for your Droplet.    true
// ssh_keys      Array           An array containing the IDs or fingerprints of the SSH keys that you wish to embed in the Droplet's root account upon creation. 
// backups       Boolean         A boolean indicating whether automated backups should be enabled for the Droplet. Automated backups can only be enabled when the Droplet is created.    
// ipv6          Boolean         A boolean indicating whether IPv6 is enabled on the Droplet.    
// private_networking  Boolean  A boolean indicating whether private networking is enabled for the Droplet. Private networking is currently only available in certain regions.  
// user_data     String          A string of the desired User Data for the Droplet. User Data is currently only available in regions with metadata listed in their features.
#[derive(Serialize)]
pub struct Droplet {
    pub name: String,
    pub region: String,
    pub size: String,
    pub image: String,
    pub ssh_keys: Option<Vec<String>>,
    pub backups: bool,
    pub ipv6: bool,
    pub private_net: bool,
    pub user_data: Option<String>
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
             if let Some(d) = self.user_data.clone() {
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
    pub fn kernels(mut self) -> RequestBuilder<'t, response::Kernels> {
        // GET: "https://api.digitalocean.com/v2/droplets/$ID/kernels"
        self.url.push('/');
        self.url.push_str("kernels");
        RequestBuilder::new(self.auth, self.url)
    }
    pub fn snapshots(mut self) -> RequestBuilder<'t, response::Snapshots> {
        // GET: "https://api.digitalocean.com/v2/droplets/$ID/snapshots"
        self.url.push('/');
        self.url.push_str("snapshots");
        RequestBuilder::new(self.auth, self.url)
    }
    pub fn backups(mut self) -> RequestBuilder<'t, response::Backups> {
        // GET: "https://api.digitalocean.com/v2/droplets/$ID/backups"
        self.url.push('/');
        self.url.push_str("backups");
        RequestBuilder::new(self.auth, self.url)
    }
    pub fn actions(mut self) -> RequestBuilder<'t, response::Actions> {
        // GET: "https://api.digitalocean.com/v2/droplets/$ID/actions"
        self.url.push('/');
        self.url.push_str("actions");
        RequestBuilder::new(self.auth, self.url)
    }
    pub fn delete(self) -> RequestBuilder<'t, response::HeaderOnly> {
        // DELETE: "https://api.digitalocean.com/v2/droplets/$ID"
        RequestBuilder {
            method: Method::Delete,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None
        }
    }
    pub fn show(&self) -> RequestBuilder<'t, response::Droplet> {
        unimplemented!()
    }
    pub fn neighbors(mut self) -> RequestBuilder<'t, response::Droplets> {
        // GET: "https://api.digitalocean.com/v2/droplets/$ID/neighbors"
        self.url.push('/');
        self.url.push_str("neighbors");
        RequestBuilder::new(self.auth, self.url)
    }
}

impl<'t> RequestBuilder<'t, response::Droplets> {
    pub fn create(self, droplet: &Droplet) -> RequestBuilder<'t, response::Droplet> {
        // POST: 
        // body:
        //      "name" : ""             // true
        //      "region" : ""           // true
        //      "size" : ""             // true
        //      "image" : ""            // true
        //      "ssh_keys" : ""        
        //      "backups" : ""         
        //      "ipv6" : ""            
        //      "private_networking" : "" 
        //      "user_data" : ""       

        // FIXME: don't unwrap()
        RequestBuilder {
            method: Method::Post,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(json::to_string(droplet).ok().unwrap())
        }

        // let mut hm = HashMap::new();
        // hm.insert("name", droplet.name.clone());
        // hm.insert("region", droplet.region.clone());
        // hm.insert("size", droplet.size.clone());
        // hm.insert("image", droplet.image.clone());
        // if let Some(ref n) = rec.name {
        //     hm.insert("name", n.to_owned());
        // }
        // if let Some(ref d) = rec.data {
        //     hm.insert("data", d.to_owned());
        // }
        // if let Some(p) = rec.priority {
        //     hm.insert("priority", p.to_string());
        // }
        // if let Some(p) = rec.port {
        //     hm.insert("name", p.to_string());
        // }
        // if let Some(w) = rec.weight {
        //     hm.insert("name", w.to_string());
        // }
    }
    pub fn neighbors(self) -> RequestBuilder<'t, response::Neighbors> {
        // GET: "https://api.digitalocean.com/v2/reports/droplet_neighbors"
        RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/reports/droplet_neighbors")
    }
    pub fn upgrades(self) -> RequestBuilder<'t, response::DropletUpgrades> {
        // GET: "https://api.digitalocean.com/v2/droplet_upgrades"
        RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/droplet_upgrades")
    }
}

impl<'t> DoRequest<response::Droplet> for RequestBuilder<'t, response::Droplet> {}

