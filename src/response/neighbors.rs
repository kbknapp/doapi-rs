use std::fmt;
use std::borrow::Cow;

use crate::response::{Backup, Image, Kernel, NamedResponse, Networks, Region, Size};

// Have to duplicate Droplet because of lack of negative trait bounds
#[derive(Deserialize, Debug)]
pub struct DropletNeighbor {
    pub id: f64,
    pub name: String,
    pub memory: f64,
    pub vcpus: f64,
    pub disk: f64,
    pub locked: bool,
    pub status: String,
    pub kernel: Option<Kernel>,
    pub created_at: String,
    pub features: Vec<String>,
    pub backup_ids: Vec<Option<f64>>,
    pub next_backup_window: Option<Backup>,
    pub snapshot_ids: Vec<Option<f64>>,
    pub image: Image,
    pub region: Region,
    pub size: Size,
    pub size_slug: String,
    pub networks: Networks,
}

impl fmt::Display for DropletNeighbor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "ID: {:.0}\n\
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
               self.backup_ids
                   .iter()
                   .filter_map(|n| {
                       if n.is_some() {
                           Some(n.clone().unwrap().to_string())
                       } else {
                           None
                       }
                   })
                   .fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]),
               self.snapshot_ids
                   .iter()
                   .filter_map(|n| {
                       if n.is_some() {
                           Some(n.clone().unwrap().to_string())
                       } else {
                           None
                       }
                   })
                   .fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]),
               self.features
                   .iter()
                   .fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]),
               &self.region.to_string()[..].replace("\n", "\n\t"),
               &self.image.to_string()[..].replace("\n", "\n\t"),
               &self.size.to_string()[..].replace("\n", "\n\t"),
               self.size_slug,
               &self.networks.to_string()[..].replace("\n", "\n\t"),
               if let Some(ref k) = self.kernel {
                   k.to_string()[..].replace("\n", "\n\t")
               } else {
                   "None".to_owned()
               },
               if let Some(ref k) = self.next_backup_window {
                   k.to_string()[..].replace("\n", "\n\t")
               } else {
                   "None".to_owned()
               })

    }
}

pub type Neighbor = Vec<DropletNeighbor>;

impl NamedResponse for Neighbor {
    fn name<'a>() -> Cow<'a, str> { "neighbor".into() }
}

pub type Neighbors = Vec<Neighbor>;
