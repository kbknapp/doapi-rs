// id                   number              A unique identifier for each Droplet instance. This is automatically generated upon Droplet creation.
// name                 string              The human-readable name set for the Droplet instance.
// memory               number              Memory of the Droplet in megabytes.
// vcpus                number              The number of virtual CPUs.
// disk                 number              The size of the Droplet's disk in gigabytes.
// locked               boolean             A boolean value indicating whether the Droplet has been locked, preventing actions by users.
// created_at           string              A time value given in ISO8601 combined date and time format that represents when the Droplet was created.
// status               string              A status string indicating the state of the Droplet instance. This may be "new", "active", "off", or "archive".
// backup_ids           array               An array of backup IDs of any backups that have been taken of the Droplet instance. Droplet backups are enabled at the time of the instance creation.
// snapshot_ids         array               An array of snapshot IDs of any snapshots created from the Droplet instance.
// features             array               An array of features enabled on this Droplet.
// region               object              The region that the Droplet instance is deployed in. When setting a region, the value should be the slug identifier for the region. When you query a Droplet, the entire region object will be returned.
// image                object              The base image used to create the Droplet instance. When setting an image, the value is set to the image id or slug. When querying the Droplet, the entire image object will be returned.
// size                 object              The current size object describing the Droplet. When setting a size, the value is set to the size slug. When querying the Droplet, the entire size object will be returned. Note that the disk volume of a droplet may not match the size's disk due to Droplet resize actions. The disk attribute on the Droplet should always be referenced.
// size_slug            string              The unique slug identifier for the size of this Droplet.
// networks             object              The details of the network that are configured for the Droplet instance. This is an object that contains keys for IPv4 and IPv6. The value of each of these is an array that contains objects describing an individual IP resource allocated to the Droplet. These will define attributes like the IP address, netmask, and gateway of the specific network depending on the type of network it is.
// kernel               nullable object     The current kernel. This will initially be set to the kernel of the base image when the Droplet is created.
// next_backup_window   nullable object     The details of the Droplet's backups feature, if backups are configured for the Droplet. This object contains keys for the start and end times of the window during which the backup will start.

use std::fmt;
use std::borrow::Cow;

use response::{Kernel, Region, Backup, Networks, Image, Size, NamedResponse};

#[derive(Deserialize, Debug)]
pub struct Droplet {
    id: f64,
    name: String,
    memory: f64,
    vcpus: f64,
    disk: f64,
    locked: bool,
    status: String,
    kernel: Option<Kernel>,
    created_at: String,
    features: Vec<String>,
    backup_ids: Vec<Option<String>>,
    next_backup_window: Option<Backup>,
    snapshot_ids: Vec<Option<String>>,
    image: Image,
    region: Region,
    size: Size,
    size_slug: String,
    networks: Networks,
}

impl fmt::Display for Droplet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "ID: {:.0}\n\
                  Name: {}\n\
                  Memory: {} MB\n\
                  Virtual CPUs: {:.0}\n\
                  Disk: {} GB\n\
                  Locked: {}\n\
                  Created At: {}\n\
                  Status: {}\n\
                  Backup IDs: {}\n\
                  Snapshot IDs: {}\n\
                  Features: {}\n\
                  Region: \n\t{}\n\
                  Image: \n\t{}\n\
                  Size: \n\t{}\n\
                  Size Slug: {}\n\
                  Network: \n\t{}\n\
                  Kernel: \n\t{}\n\
                  Next Backup Window: {}\n",
                self.id,
                self.name,
                self.memory,
                self.vcpus,
                self.disk,
                self.locked,
                self.created_at,
                self.status,
                self.backup_ids.iter()
                               .filter_map(|n| if n.is_some() {
                                  Some(n.clone().unwrap().to_string())
                               }else{
                                  None
                               })
                               .fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]),
                self.snapshot_ids.iter()
                                 .filter_map(|n| if n.is_some() {
                                    Some(n.clone().unwrap().to_string())
                                 }else{
                                    None
                                 })
                                 .fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]),
                self.features.iter()
                             .fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]),
                &self.region.to_string()[..].replace("\n","\n\t"),
                &self.image.to_string()[..].replace("\n","\n\t"),
                &self.size.to_string()[..].replace("\n","\n\t"),
                self.size_slug,
                &self.networks.to_string()[..].replace("\n","\n\t"),
                if let Some(ref k) = self.kernel {
                    format!("{}", &k.to_string()[..].replace("\n","\n\t"))
                } else {
                    "None".to_owned()
                },
                if let Some(ref k) = self.next_backup_window {
                    format!("{}", &k.to_string()[..].replace("\n","\n\t"))
                } else {
                    "None".to_owned()
                })

    }
}

impl NamedResponse for Droplet {
    fn name<'a>() -> Cow<'a, str> {
        "droplet".into()
    }
}

pub type Droplets = Vec<Droplet>;

