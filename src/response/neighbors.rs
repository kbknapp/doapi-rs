
use std::fmt;
use std::borrow::Cow;

use response::{Kernel, Region, Backup, Networks, Image, Size, NamedResponse};

// Have to duplicate Droplet because of lack of negative trait bounds
#[derive(Deserialize, Debug)]
pub struct DropletNeighbor {
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

impl fmt::Display for DropletNeighbor {
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

pub type Neighbor = Vec<DropletNeighbor>;

impl NamedResponse for Neighbor {
    fn name<'a>() -> Cow<'a, str> {
        "neighbor".into()
    }
}

pub type Neighbors = Vec<Neighbor>;
