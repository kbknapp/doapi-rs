Please use the following template to assist with creating an issue, and getting a speedy resolution. If an area is not aplicable, feel free to delete the area, or mark with `N/A`

### Rust Version

* Use the output of `rustc -V`

### Affected Version of doapi-rs

* Can be found in Cargo.lock of your project (i.e. `grep doapi Cargo.lock`)

### Expected Behavior Summary


### Actual Behavior Summary


### Steps to Reproduce the issue


### Sample Code or Link to Sample Code


### Debug output

Compile doapi with cargo features `"debug"` such as:

```toml
[dependencies]
doapi = { version = "~0.1.1", features = ["debug"] }
```
The output may be very long, so feel free to link to a gist or attach a text file