# doapi-rs
[![Travis](https://img.shields.io/travis/kbknapp/doapi-rs.svg)](https://travis-ci.org/kbknapp/doapi-rs) [![GitHub license](https://img.shields.io/github/license/kbknapp/doapi-rs.svg)](https://github.com/kbknapp/doapi-rs) [![GitHub release](https://img.shields.io/github/release/kbknapp/doapi-rs.svg)](https://github.com/kbknapp/doapi-rs) [![Join the chat at https://gitter.im/kbknapp/doapi-rs](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/kbknapp/doapi-rs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Wrapper library for utilizing [DigitalOcean API v2](https://developers.digitalocean.com/documentation/) in Rust

## Disclaimer
This library is very alpha - it may do anything up to, and including, eating your laundry.

## Example

The following example takes a snapshot of an existing droplet, which in turn sends back an `Action`.

```rust
let auth_token = "INSERT AUTH TOKEN HERE";
let domgr = DoManager::with_auth(&auth_token);
let snapshot_name = "my new snapshot";
let droplet_id = "1234567";

print!("Sending request...")
match domgr.droplet(droplet_id)
           .snapshot(snapshot_name)
           .retrieve() {
Ok(action) => {
    println!("Success!\n\t");
    println!("{}\n", action);
},
Err(e) => {
    pritnln!("Failed\n\t.")
    println!("{}\n", e);
}
```

This library can be used to send the request and retrieve the object back from DigitalOcean all at once (as in the above example), or you use this library to build the request and change it to your liking. For example (still using the above as a base), you could see the request being sent to DigitalOcean without sending it via:

```rust
domgr.droplet(droplet_id)
     .snapshot(snapshot_name)
     .to_string();
```

You can also get the raw JSON back from DigitalOcean

```rust
print!("Sending request...")
match domgr.droplet(droplet_id)
           .snapshot(snapshot_name)
           .retrieve_json() {
Ok(json) => {
    println!("Success!\n\t");
    println!("{}\n", json);
},
Err(e) => {
    pritnln!("Failed\n\t.")
    println!("{}\n", e);
}
```

You can also get the raw `hyper` requests and responses for your manipulation.

## Usage

At the moment, `doapi` requeires a nightly Rust compiler.

Add `doapi` as a dependecy in your `Cargo.toml` file to use from crates.io:

 ```toml
 [dependencies]
 doapi = "*"
 ```
 Or track the latest on the master branch at github:

```toml
[dependencies.doapi]
git = "https://github.com/kbknapp/doapi-rs.git"
```

Add `extern crate dopai;` to your crate root.

### DigitalOcean Personal Auth Token

In order to use the DigitalOcean v2 API, you must generate a Personal Authentication Token. This token can then be passed to the `DoManager` in order to build requests and retrieve results.

Personal Auth Token's can be Read/Write, or Read Only/Write Only. In order to process destructive API calls (such as various `.delete()`, `.create()`, `.update()` etc.) you *must* have a token with Write priviledges.

To generate a new Personal Auth Token see the [following DigitalOcean details](https://developers.digitalocean.com/documentation/v2/#authentication)

### More Information

You can find complete documentation on the [github-pages site](http://kbknapp.github.io/doapi-rs/index.html) for this project.

## Contributing

Contributions are always welcome! And there is a multitude of ways in which you can help depending on what you like to do, or are good at. Anything from documentation, code cleanup, issue completion, new features, you name it, even filing issues is contributing and greatly appreciated!

**NOTE:** One of the best ways to help right now is to simply use the library and report issues!

1. Fork `doapi`
2. Clone your fork (`git clone https://github.com/$YOUR_USERNAME/doapi-rs && cd doapi-rs`)
3. Create new branch (`git checkout -b new-branch`)
4. Make your changes, and commit (`git commit -am "your message"`)
 * I use a [conventional](https://github.com/ajoslin/conventional-changelog/blob/master/CONVENTIONS.md) changelog format so I can update my changelog using [clog](https://github.com/thoughtram/clog)
 * Format your commit subject line using the following format: `TYPE(COMPONENT): MESSAGE` where `TYPE` is one of the following:
    - `feat` - A new feature
    - `imp` - An improvement to an existing feature
    - `perf` - A performance improvement
    - `docs` - Changes to documentation only
    - `tests` - Changes to the testing framework or tests only
    - `fix` - A bug fix
    - `refactor` - Code functionality doesn't change, but underlying structure may
    - `style` - Stylistic changes only, no functionality changes
    - `wip` - A work in progress commit (Should typically be `git rebase`'ed away)
    - `chore` - Catch all or things that have to do with the build system, etc
 * The `COMPONENT` is optional, and may be a single file, directory, or logical component. Can be omitted if commit applies globally
5. `git rebase` into concise commits and remove `--fixup`s (`git rebase -i HEAD~NUM` where `NUM` is number of commits back)
6. Push your changes back to your fork (`git push origin $your-branch`)
7. Create a pull request! (You can also create the pull request first, and we'll merge when ready. This a good way to discuss proposed changes.)

## Recent Breaking Changes

Although I do my best to keep breaking changes to a minimum, being that this a sub 1.0 library, there are breaking changes from time to time in order to support better features or implementation. For the full details see the changelog.md

**NONE Yay :)**

### Deprecations

Old method names will be left around for some time.

**NONE Yay :)**
