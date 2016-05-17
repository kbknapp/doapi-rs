use std::fmt;

use response;

#[derive(Deserialize, Debug, Clone)]
pub struct Network {
    pub ip_address: String,
    pub netmask: String,
    pub gateway: String,
    #[serde(rename = "type")]
    pub network_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Networks {
    pub v4: Vec<Option<Network>>,
    pub v6: Vec<Option<Network>>,
}

impl response::NotArray for Network {}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "IP Address: {}\n\
                  Netmask: {}\n\
                  Gateway:{}\n\
                  Type: {}",
               self.ip_address,
               self.netmask,
               self.gateway,
               self.network_type)
    }
}

impl fmt::Display for Networks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IPv4 Networks: {}\n\
                  IPv6 Networks: {}",
            {
                let v4 = self.v4
                             .iter()
                             .filter_map(|n| if n.is_some() {
                                Some(n.clone().unwrap().to_string())
                             }else{
                                None
                             })
                             .fold(String::new(),|acc, n| {
                                acc + &n.replace("\n", "\n\t")
                             });
                if !v4.is_empty() {
                    format!("\n\t{}", v4)
                } else {
                    "None".to_owned()
                }
            },
            {
                let v6 = self.v6
                             .iter()
                             .filter_map(|n| if n.is_some() {
                                Some(n.clone().unwrap().to_string())
                             }else{
                                None
                             })
                             .fold(String::new(),|acc, n| {
                                acc + &n.replace("\n", "\n\t")
                             });
                if !v6.is_empty() {
                    format!("\n\t{}", v6)
                } else {
                    "None".to_owned()
                }
            },
        )
    }
}
