// id               number              A unique number that can be used to
// identify and reference a specific image.
// name             string              The display name that has been given to
// an image. This is what is shown in the control panel and is generally a
// descriptive title for the image in question.
// type             string              The kind of image, describing the
// duration of how long the image is stored. This is one of "snapshot",
// "temporary" or "backup".
// distribution     string              This attribute describes the base
// distribution used for this image.
// slug             nullable string     A uniquely identifying string that is
// associated with each of the DigitalOcean-provided public images. These can
// be used to reference a public image as an alternative to the numeric id.
// public           boolean             This is a boolean value that indicates
// whether the image in question is public or not. An image that is public is
// available to all accounts. A non-public image is only accessible from your
// account.
// regions          array               This attribute is an array of the
// regions that the image is available in. The regions are represented by their
// identifying slug values.
// created_at       String
// min_disk_size    number              The minimum 'disk' required for a size
// to use this image.

use std::fmt;
use std::borrow::Cow;

use response::NamedResponse;
use response;

#[derive(Deserialize, Debug)]
pub struct Image {
    pub id: f64,
    pub name: String,
    pub distribution: String,
    pub slug: Option<String>,
    pub public: bool,
    pub regions: Vec<String>,
    pub created_at: String,
    pub min_disk_size: f64,
    #[serde(rename = "type")]
    pub image_type: String,
}

impl response::NotArray for Image {}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "ID: {:.0}\n\
                  Name: {}\n\
                  Type: {}\n\
                  Distribution: {}\n\
                  Slug: {}\n\
                  Public: {} MB\n\
                  Regions: {}\n\
                  Minimum Disk Size: {} GB",
               self.id,
               self.name,
               self.image_type,
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

impl NamedResponse for Image {
    fn name<'a>() -> Cow<'a, str> { "image".into() }
}

pub type Images = Vec<Image>;
