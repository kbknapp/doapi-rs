// id               number      A unique number used to identify and reference
// a specific image.
// name             string      The display name of the image. This is shown in
// the web UI and is generally a descriptive title for the image in question.
// type             string      The kind of image, describing the duration of
// how long the image is stored. This is one of "snapshot", "temporary" or
// "backup".
// distribution     string      The base distribution used for this image.
// slug             nullable string A uniquely identifying string that is
// associated with each of the DigitalOcean-provided public images. These can
// be used to reference a public image as an alternative to the numeric id.
// public           boolean     A boolean value that indicates whether the
// image in question is public. An image that is public is available to all
// accounts. A non-public image is only accessible from your account.
// regions          array       An array of the regions that the image is
// available in. The regions are represented by their identifying slug values.
// min_disk_size    number      The minimum 'disk' required for a size to use
// this image.

use std::fmt;
use std::borrow::Cow;

use response::NamedResponse;
use response;

#[derive(Deserialize, Debug)]
pub struct Backup {
    pub id: f64,
    pub name: String,
    #[serde(rename = "type")]
    pub b_type: String,
    pub distribution: String,
    pub slug: Option<String>,
    pub public: bool,
    pub regions: Vec<String>,
    pub min_disk_size: f64,
}

impl response::NotArray for Backup {}

impl fmt::Display for Backup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "ID: {:.0}\n\
                  Name: {}\n\
                  Type:{}\n\
                  Distribution:{}\n\
                  Slug:{}\n\
                  Public:{}\n\
                  Regions:{}\n\
                  Minimum Disk Size: {:.0} MB\n",
               self.id,
               self.name,
               self.b_type,
               self.distribution,
               if let Some(ref s) = self.slug {
                   s.clone()
               } else {
                   "None".to_owned()
               },
               self.public,
               self.regions.iter().fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]),
               self.min_disk_size)
    }
}

pub type Backups = Vec<Backup>;

impl NamedResponse for Backup {
    fn name<'a>() -> Cow<'a, str> { "backup".into() }
}
