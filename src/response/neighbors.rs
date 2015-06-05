
use std::fmt;
use std::borrow::Cow;

use response::{self, Kernel, Region, Backup, Network, Image, Size, NamedResponse};

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

impl response::NotArray for Neighbor {}

impl fmt::Display for Neighbor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Droplet:\n\
                        ID: {:.0}\n\
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
                        Droplet: \n{}\n\
                        Image: \n{}\n\
                        Size: \n{}\n\
                        Size Slug: {}\n\
                        Network: \n{}\n\
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
                    format!("\n{}\n", k)
                } else {
                    "None\n".to_owned()
                },
                if let Some(ref k) = self.next_backup_window {
                    format!("\n{}\n", k)
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