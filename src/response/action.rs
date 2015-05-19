use std::slice::Iter;
use std::fmt;

use response::meta::Meta;
use response::region::Region;
use response::links::Links;

#[derive(Deserialize, Debug)]
pub struct Action {
    id: f64,
    status: String,
    #[serde(rename="type")]
    action_type: String,
    started_at: String,
    completed_at: String,
    resource_id: f64,
    resource_type: String,
    region: Region,
    region_slug: Option<String>
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Account Action:\n\t\
                        ID: {}\n\t\
                        Status: {}\n\t\
                        Type: {}\n\t\
                        Started At: {}\n\t\
                        Completed At: {}\n\t\
                        Resource ID: {}\n\t\
                        Resource Type: {}\n\t\
                        {}\
                        Region Slug: {}\n",
                self.id,
                self.status,
                self.action_type,
                self.started_at,
                self.completed_at,
                self.resource_id,
                self.resource_type,
                self.region.to_string().replace("\n", "\n\t"),
                if self.region_slug.is_some() { self.region_slug.clone().unwrap() } else { "None".to_owned() })
    }
}

pub struct Actions {
    pub actions: Vec<Action>,
}

impl Actions {
    pub fn iter(&self) -> Iter<Action> {
        self.actions.iter()
    }
}

#[derive(Deserialize)]
pub struct RawActions {
    pub actions: Vec<Action>,
    pub links: Links,
    pub meta: Meta
}

//{"actions":
//  [{  "id":48658518,
//      "status":"completed",
//      "type":"power_on",
//      "started_at":"2015-04-21T17:02:51Z",
//      "completed_at":"2015-04-21T17:02:56Z",
//      "resource_id":3193993,
//      "resource_type":"droplet",
//      "region":
//          {"name":"New York 3",
//          "slug":"nyc3",
//          "sizes":["512mb","1gb","2gb","4gb","8gb","16gb","32gb","48gb","64gb"],
//          "features":["virtio","private_networking","backups","ipv6","metadata"],
//          "available":true},
//      "region_slug":"nyc3"},
//"links":
//  {"pages":
//      {"last":"https://api.digitalocean.com/v2/actions?page=2",
//      "next":"https://api.digitalocean.com/v2/actions?page=2"}},
//"meta":{"total":24}}
