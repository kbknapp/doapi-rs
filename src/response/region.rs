// slug         string      A human-readable string that is used as a unique
// identifier for each region.
// name         string      The display name of the region. This will be a full
// name that is used in the control panel and other interfaces.
// sizes        array       This attribute is set to an array which contains
// the identifying slugs for the sizes available in this region.
// available    boolean     This is a boolean value that represents whether new
// Droplets can be created in this region.
// features     array       This attribute is set to an array which contains
// features available in this region

use std::fmt;
use std::borrow::Cow;

use response::NamedResponse;
use response;

#[derive(Deserialize, Debug)]
pub struct Region {
    name: String,
    slug: String,
    sizes: Vec<String>,
    features: Vec<String>,
    available: bool,
}

impl response::NotArray for Region {}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Name: {}\n\
                  Slug: {}\n\
                  Sizes:{}\n\
                  Features:{}\n\
                  Available: {}",
               self.name,
               self.slug,
               self.sizes.iter().fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]),
               self.features.iter().fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]),
               self.available)
    }
}

impl NamedResponse for Region {
    fn name<'a>() -> Cow<'a, str> {
        "region".into()
    }
}

pub type Regions = Vec<Region>;
