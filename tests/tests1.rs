use std::thread::sleep;
use std::time::Duration;
use x509_test_framework_dev::server;

/*
docker run --rm --env-file password-local.env -p 127.0.0.1:8080:80 -p 127.0.0.1:4443:443 $(docker build -q --secret id=password,src=password.txt --secret id=password-ca,src=password.txt docker)
 */
#[test]
fn test1() {
    server::start("test".to_string(), "127.0.0.1".parse().unwrap(), 8888, 4444).unwrap();
    sleep(Duration::from_secs(1));
}

#[test]
fn test2() {
    server::start("test".to_string(), "127.0.0.1".parse().unwrap(), 8888, 4444).unwrap();
    sleep(Duration::from_secs(1));
}
#[test]
fn test3() {
    server::start("test".to_string(), "127.0.0.1".parse().unwrap(), 8888, 4444).unwrap();
    sleep(Duration::from_secs(1));
}
#[test]
fn test4() {
    server::start("test".to_string(), "127.0.0.1".parse().unwrap(), 8888, 4444).unwrap();
    sleep(Duration::from_secs(1));
}
