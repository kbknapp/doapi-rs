#[derive(Deserialize, Debug)]
pub struct Action {
    id: f64,
    status: String,
    /// Real json is "type"
    action_type: String,
    started_at: String,
    completed_at: String,
    resource_id: f64,
    resource_type: String,
    region: Option<String>,
    region_slug: Option<String>
}
//{
//   "actions": [
//     {
//       "id": 36804636,
//       "status": "completed",
//       "type": "create",
//       "started_at": "2014-11-14T16:29:21Z",
//       "completed_at": "2014-11-14T16:30:06Z",
//       "resource_id": 3164444,
//       "resource_type": "droplet",
//       "region": "nyc3",
//       "region_slug": "nyc3"
//     }
//   ],
//   "links": {
//     "pages": {
//       "last": "https://api.digitalocean.com/v2/actions?page=159&per_page=1",
//       "next": "https://api.digitalocean.com/v2/actions?page=2&per_page=1"
//     }
//   },
//   "meta": {
//     "total": 159
//   }
// }
