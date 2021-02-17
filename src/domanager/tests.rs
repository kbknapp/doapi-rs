use super::*;

use httpmock::Method::GET;
use httpmock::{HttpMockRequest, MockServer};

use crate::tests::{BEARER_HEADER, TEST_TOKEN};
use crate::DoRequest;

#[test]
fn test_do_manager() {
    let url = Url::parse("https://api.digitalocean.com/v2/").unwrap();
    let domgr = DoManager::with_token(TEST_TOKEN);

    assert_eq!(domgr.endpoint_url, url);
    assert_eq!(domgr.auth, TEST_TOKEN);
}

#[test]
fn test_account() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/account")
            .header("Authorization", &BEARER_HEADER);

        then.status(200)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("ratelimit-limit", "1200")
            .header("ratelimit-remaining", "1137")
            .header("ratelimit-reset", "1415984218")
            .body(include_str!("test_account_response.json"));
    });

    let url = Url::parse(&server.base_url()).unwrap();

    let domgr = DoManager::builder()
        .token(TEST_TOKEN)
        .endpoint_url(url)
        .build();

    let account = domgr.account().retrieve().unwrap();

    assert_eq!(account.droplet_limit as u32, 25);
    assert_eq!(account.floating_ip_limit as u32, 5);
    assert_eq!(account.volume_limit as u32, 10);
    assert_eq!(account.email, "sammy@digitalocean.com");
    assert_eq!(account.uuid, "b6fr89dbf6d9156cace5f3c78dc9851d957381ef");
    assert_eq!(account.email_verified, true);
    assert_eq!(account.status, "active");
    assert_eq!(account.status_message, "");

    mock.assert();
}

#[test]
fn test_image() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/image/7555620")
            .header("Authorization", &BEARER_HEADER);

        then.status(200)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("ratelimit-limit", "1200")
            .header("ratelimit-remaining", "774")
            .header("ratelimit-reset", "1415984218")
            .body(include_str!("test_image_response.json"));
    });

    let url = Url::parse(&server.base_url()).unwrap();

    let domgr = DoManager::builder()
        .token(TEST_TOKEN)
        .endpoint_url(url)
        .build();

    let image = domgr.image("7555620").retrieve().unwrap();
    assert_eq!(image.id as u32, 7555620);
    assert_eq!(image.name, "Nifty New Snapshot");
    assert_eq!(image.distribution, "Ubuntu");
    assert_eq!(image.slug, None);
    assert_eq!(image.public, false);
    assert_eq!(image.regions[0], "nyc2");
    assert_eq!(image.regions[1], "nyc2");
    assert_eq!(image.created_at, "2014-11-04T22:23:02Z");
    assert_eq!(image.min_disk_size as u32, 20);
    assert_eq!(image.description, "");
    assert!(image.tags.is_empty());
    assert_eq!(image.status, "available");
    assert_eq!(image.error_message, "");

    mock.assert();
}

#[test]
fn test_images() {
    let server = MockServer::start();

    let response_json = include_str!("test_images_response.json");

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/images")
            .header("Authorization", &BEARER_HEADER);

        then.status(200)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("ratelimit-limit", "1200")
            .header("ratelimit-remaining", "777")
            .header("ratelimit-reset", "1415984218")
            .body(response_json);
    });

    let url = Url::parse(&server.base_url()).unwrap();

    let domgr = DoManager::builder()
        .token(TEST_TOKEN)
        .endpoint_url(url)
        .build();

    let domains = domgr.images().retrieve().unwrap();
    assert_eq!(domains.len(), 2);

    mock.assert();
}

#[test]
fn test_domain() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/domains/example.com")
            .header("Authorization", &BEARER_HEADER);

        then.status(200)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("ratelimit-limit", "1200")
            .header("ratelimit-remaining", "1112")
            .header("ratelimit-reset", "1415984218")
            .body(include_str!("test_domain_response.json"));
    });

    let url = Url::parse(&server.base_url()).unwrap();

    let domgr = DoManager::builder()
        .token(TEST_TOKEN)
        .endpoint_url(url)
        .build();

    let domain = domgr.domain("example.com").retrieve().unwrap();
    assert_eq!(domain.name, "example.com");
    assert_eq!(domain.ttl as u32, 1800);

    mock.assert();
}

#[test]
fn test_domains() {
    let server = MockServer::start();

    let response_json = include_str!("test_domains_response.json");

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/domains")
            .header("Authorization", &BEARER_HEADER);

        then.status(200)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("ratelimit-limit", "1200")
            .header("ratelimit-remaining", "767")
            .header("ratelimit-reset", "1415984218")
            .body(response_json);
    });

    let url = Url::parse(&server.base_url()).unwrap();

    let domgr = DoManager::builder()
        .token(TEST_TOKEN)
        .endpoint_url(url)
        .build();

    let domains = domgr.domains().retrieve().unwrap();
    assert_eq!(domains.len(), 2);

    mock.assert();
}

#[test]
fn test_ssh_key() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/account/keys/512190")
            .header("Authorization", &BEARER_HEADER);

        then.status(200)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("ratelimit-limit", "1200")
            .header("ratelimit-remaining", "764")
            .header("ratelimit-reset", "1415984218")
            .body(include_str!("test_ssh_key_response.json"));
    });

    let url = Url::parse(&server.base_url()).unwrap();

    let domgr = DoManager::builder()
        .token(TEST_TOKEN)
        .endpoint_url(url)
        .build();

    let ssh_key = domgr.ssh_key("512190").retrieve().unwrap();
    assert_eq!(ssh_key.id as u32, 512190);
    assert_eq!(
        ssh_key.fingerprint,
        "3b:16:bf:e4:8b:00:8b:b8:59:8c:a9:d3:f0:19:45:fa"
    );
    assert_eq!(ssh_key.public_key, "ssh-rsa AEXAMPLEaC1yc2EAAAADAQABAAAAQQDDHr/jh2Jy4yALcK4JyWbVkPRaWmhck3IgCoeOO3z1e2dBowLh64QAM+Qb72pxekALga2oi4GvT+TlWNhzPH4V example");
    assert_eq!(ssh_key.name, "My SSH Public Key");

    mock.assert();
}

#[test]
fn test_ssh_keys() {
    // Testing multiple responses here to ensure that the paged response API gets covered.
    
    let server = MockServer::start();

    let response1_json =
        include_str!("test_ssh_keys_response1.json").replace("base_url", &server.base_url());
    let response2_json =
        include_str!("test_ssh_keys_response2.json").replace("base_url", &server.base_url());
    let response3_json =
        include_str!("test_ssh_keys_response3.json").replace("base_url", &server.base_url());
    let mock1 = server.mock(|when, then| {
        when.method(GET)
            .path("/account/keys")
            .matches(|request: &HttpMockRequest| match &request.query_params {
                Some(query_params) => query_params.is_empty(),
                None => false,
            })
            .header("Authorization", &BEARER_HEADER);

        then.status(200)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("ratelimit-limit", "1200")
            .header("ratelimit-remaining", "767")
            .header("ratelimit-reset", "1415984218")
            .body(response1_json);
    });

    let mock2 = server.mock(|when, then| {
        when.method(GET)
            .path("/account/keys")
            .query_param("page", "2")
            .header("Authorization", &BEARER_HEADER);

        then.status(200)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("ratelimit-limit", "1200")
            .header("ratelimit-remaining", "766")
            .header("ratelimit-reset", "1415984218")
            .body(response2_json);
    });

    let mock3 = server.mock(|when, then| {
        when.method(GET)
            .path("/account/keys")
            .query_param("page", "3")
            .header("Authorization", &BEARER_HEADER);

        then.status(200)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("ratelimit-limit", "1200")
            .header("ratelimit-remaining", "765")
            .header("ratelimit-reset", "1415984218")
            .body(response3_json);
    });

    let url = Url::parse(&server.base_url()).unwrap();

    let domgr = DoManager::builder()
        .token(TEST_TOKEN)
        .endpoint_url(url)
        .build();

    let ssh_keys = domgr.ssh_keys().retrieve().unwrap();
    assert_eq!(ssh_keys.len(), 43);
    mock1.assert();
    mock2.assert();
    mock3.assert();
}
