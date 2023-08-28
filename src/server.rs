use std::io;
use std::io::ErrorKind;
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::str::from_utf8;

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref IDENTITY_SERVER_INNER: Arc<Mutex<Option<IdentityServerInner>>> =
        Arc::new(Mutex::new(None));
}

pub fn start(password: String, host: Ipv4Addr, port: usize, port_ssl: usize) -> io::Result<()> {
    if let Ok(mut server) = IDENTITY_SERVER_INNER.lock() {
        if server.is_none() {
            let context = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docker");

            *server = Some(IdentityServerInner::start(
                &context, password, host, port, port_ssl,
            )?);
        }
    }
    Ok(())
}

pub fn stop() -> io::Result<()> {
    if let Some(server) = IDENTITY_SERVER_INNER
        .lock()
        .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?
        .as_mut()
    {
        server.stop()?;
    }
    Ok(())
}

struct IdentityServerInner {
    process: Child,
}

impl IdentityServerInner {
    fn start(
        context: &Path,
        password: String,
        host: Ipv4Addr,
        port: usize,
        port_ssl: usize,
    ) -> io::Result<Self> {
        eprintln!("starting server");

        let build = Command::new("docker")
            .env("TLS_KEY_PASSWORD", &password)
            .args([
                "build",
                "-q",
                "--secret",
                "id=password,env=TLS_KEY_PASSWORD",
                "--secret",
                "id=password-ca,env=TLS_KEY_PASSWORD",
                &context.to_string_lossy(),
            ])
            .output()?;

        let build = from_utf8(&build.stdout)
            .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?
            .trim();

        println!("docker build: {}", build);

        Ok(Self {
            process: Command::new("docker")
                .env("ENCRYPTED_TLS_KEY_PASSWORD", &password)
                .args([
                    "run",
                    "--rm",
                    "--env",
                    "ENCRYPTED_TLS_KEY_PASSWORD",
                    "-p",
                    format!("{}:{}:80", host, port).as_str(),
                    "-p",
                    format!("{}:{}:443", host, port_ssl).as_str(),
                    build,
                ])
                .spawn()?,
        })
    }

    fn stop(&mut self) -> io::Result<()> {
        eprintln!("stopping server");
        self.process.kill()
    }
}

impl Drop for IdentityServerInner {
    fn drop(&mut self) {
        self.stop().expect("failed to shut down server")
    }
}
