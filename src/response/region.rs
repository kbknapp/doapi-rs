use std::fmt;

//      "region":
//          {"name":"New York 3",
//          "slug":"nyc3",
//          "sizes":["512mb","1gb","2gb","4gb","8gb","16gb","32gb","48gb","64gb"],
//          "features":["virtio","private_networking","backups","ipv6","metadata"],
//          "available":true},
#[derive(Deserialize, Debug)]
pub struct Region {
    name: String,
    slug: String,
    sizes: Vec<String>,
    features: Vec<String>,
    available: bool
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Region:\n\t\
                        Name: {}\n\t\
                        Slug: {}\n\t\
                        Sizes:{}\n\t\
                        Features:{}\n\t\
                        Available: {}\n",
                self.name,
                self.slug,
                self.sizes.iter().fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]),
                self.features.iter().fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]),
                self.available)
    }
}
