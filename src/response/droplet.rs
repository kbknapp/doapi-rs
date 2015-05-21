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

use response::{Kernel, Backup, Network, Region, Image, Size};

#[derive(Deserialize, Debug)]
pub struct Droplet {
    id: f64,
    name: String,
    memory: f64,
    vcpus: f64,
    disk: f64,
    locked: bool,
    created_at: String,
    status: String,
    backup_ids: Vec<String>,
    snapshot_ids: Vec<String>,
    features: Vec<String>,
    region: Region,
    image: Image,
    size: Size,
    size_slug: String,
    networks: Network,
    kernel: Option<Kernel>,
    next_backup_window: Option<Backup>
}

impl fmt::Display for Droplet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Droplet:\n\t\
                        ID: {:.0}\n\t\
                        Name: {}\n\t\
                        Memory: {} MB\n\t\
                        Virtual CPUs: {:.0}\n\t\
                        Disk: {} GB\n\t\
                        Locked: {}\n\t\
                        Created At: {}\n\t\
                        Status: {}\n\t\
                        Backup IDs: {}\n\t\
                        Snapshot IDs: {}\n\t\
                        Features: {}\n\t\
                        Region: \n\t{}\n\
                        Image: \n\t{}\n\
                        Size: \n\t{}\n\
                        Size Slug: {}\n\t\
                        Network: \n\t{}\n\
                        Kernel: {}\
                        Next Backup Window: {}",
                self.id,
                self.name,
                self.memory,
                self.vcpus,
                self.disk,
                self.locked,
                self.created_at,
                self.status,
                self.backup_ids.iter().fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]),
                self.snapshot_ids.iter().fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]),
                self.features.iter().fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]),
                self.region,
                self.image,
                self.size,
                self.size_slug,
                self.networks,
                if let Some(ref k) = self.kernel {
                    format!("\n\t{}\n", k)
                } else {
                    "None\n\t".to_owned()
                },
                if let Some(ref k) = self.next_backup_window {
                    format!("\n\t{}\n", k)
                } else {
                    "None\n".to_owned()
                })
                
    }
}