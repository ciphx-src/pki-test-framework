use crate::server::IdentityServer;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

#[macro_use]
extern crate lazy_static;

mod server;

/*
docker run --rm --env-file password-local.env -p 127.0.0.1:8080:80 -p 127.0.0.1:4443:443 $(docker build -q --secret id=password,src=password.txt --secret id=password-ca,src=password.txt docker)
 */
#[test]
fn test1() {
    IdentityServer::start(
        &PathBuf::from_str("/Users/curtisleefulton/merlin/x509-test-framework-dev/docker").unwrap(),
        &PathBuf::from_str("/Users/curtisleefulton/merlin/x509-test-framework-dev/password.txt")
            .unwrap(),
        &PathBuf::from_str("/Users/curtisleefulton/merlin/x509-test-framework-dev/password.txt")
            .unwrap(),
        Ipv4Addr::from_str("127.0.0.1").unwrap(),
        8080,
        4443,
    )
    .unwrap();
    sleep(Duration::from_secs(1));
}

#[test]
fn test2() {
    IdentityServer::start(
        &PathBuf::from_str("/Users/curtisleefulton/merlin/x509-test-framework-dev/docker").unwrap(),
        &PathBuf::from_str("/Users/curtisleefulton/merlin/x509-test-framework-dev/password.txt")
            .unwrap(),
        &PathBuf::from_str("/Users/curtisleefulton/merlin/x509-test-framework-dev/password.txt")
            .unwrap(),
        Ipv4Addr::from_str("127.0.0.1").unwrap(),
        8080,
        4443,
    )
    .unwrap();
    sleep(Duration::from_secs(1));
}
#[test]
fn test3() {
    IdentityServer::start(
        &PathBuf::from_str("/Users/curtisleefulton/merlin/x509-test-framework-dev/docker").unwrap(),
        &PathBuf::from_str("/Users/curtisleefulton/merlin/x509-test-framework-dev/password.txt")
            .unwrap(),
        &PathBuf::from_str("/Users/curtisleefulton/merlin/x509-test-framework-dev/password.txt")
            .unwrap(),
        Ipv4Addr::from_str("127.0.0.1").unwrap(),
        8080,
        4443,
    )
    .unwrap();
    sleep(Duration::from_secs(1));
}
#[test]
fn test4() {
    IdentityServer::start(
        &PathBuf::from_str("/Users/curtisleefulton/merlin/x509-test-framework-dev/docker").unwrap(),
        &PathBuf::from_str("/Users/curtisleefulton/merlin/x509-test-framework-dev/password.txt")
            .unwrap(),
        &PathBuf::from_str("/Users/curtisleefulton/merlin/x509-test-framework-dev/password.txt")
            .unwrap(),
        Ipv4Addr::from_str("127.0.0.1").unwrap(),
        8080,
        4443,
    )
    .unwrap();
    sleep(Duration::from_secs(1));
}
