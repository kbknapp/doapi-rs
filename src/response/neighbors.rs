
use std::fmt;
use std::borrow::Cow;

use response::{Kernel, Region, Backup, Network, Image, Size, NamedResponse};

pub type Neighbors = Vec<Neighbor>;

#[derive(Deserialize, Debug)]
pub struct Neighbor {
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

impl fmt::Display for Neighbor {
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
                        Droplet: \n\t{}\n\
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

impl NamedResponse for Neighbor {
    fn name<'a>() -> Cow<'a, str> {
        "neighbor".into()
    }
}