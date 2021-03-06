// slug             string      A human-readable string that is used to
// uniquely identify each size.
// available        boolean     This is a boolean value that represents whether
// new Droplets can be created with this size.
// transfer         number      The amount of transfer bandwidth that is
// available for Droplets created in this size. This only counts traffic on the
// public interface. The value is given in terabytes.
// price_monthly    number      This attribute describes the monthly cost of
// this Droplet size if the Droplet is kept for an entire month. The value is
// measured in US dollars.
// price_hourly     number      This describes the price of the Droplet size as
// measured hourly. The value is measured in US dollars.
// memory           number      The amount of RAM allocated to Droplets created
// of this size. The value is represented in megabytes.
// vcpus            number      The number of virtual CPUs allocated to
// Droplets of this size.
// disk             number      The amount of disk space set aside for Droplets
// of this size. The value is represented in gigabytes.
// regions          array       An array containing the region slugs where this
// size is available for Droplet creates.

use std::fmt;
use std::borrow::Cow;

use response::NamedResponse;
use response;

#[derive(Deserialize, Debug)]
pub struct Size {
    pub slug: String,
    pub memory: f64,
    pub vcpus: f64,
    pub disk: f64,
    pub transfer: f64,
    pub price_monthly: f64,
    pub price_hourly: f64,
    pub regions: Vec<String>,
    pub available: bool,
}

impl response::NotArray for Size {}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Slug: {}\n\
                  Available: {}\n\
                  Transfer Bandwidth: {} TB\n\
                  Monthly Price: ${}\n\
                  Hourly Price: ${}\n\
                  Memory: {} MB\n\
                  Virtual CPUs: {:.0}\n\
                  Disk Space: {} GB\n\
                  Sizes: {}",
               self.slug,
               self.available,
               self.transfer,
               self.price_monthly,
               self.price_hourly,
               self.memory,
               self.vcpus,
               self.disk,
               self.regions.iter().fold(String::new(), |acc, s| acc + &format!(" {},", s)[..]))

    }
}

impl NamedResponse for Size {
    fn name<'a>() -> Cow<'a, str> { "size".into() }
}

pub type Sizes = Vec<Size>;
