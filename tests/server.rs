use lazy_static::lazy_static;
use std::io;
use std::io::ErrorKind;
use std::net::Ipv4Addr;
use std::path::Path;
use std::process::{Child, Command};
use std::str::from_utf8;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref IDENTITY_SERVER_INNER: Arc<Mutex<Option<IdentityServerInner>>> =
        Arc::new(Mutex::new(None));
}

pub struct IdentityServer;

impl IdentityServer {
    pub fn start(
        context: &Path,
        tls_password: &Path,
        tls_ca_password: &Path,
        host: Ipv4Addr,
        port: usize,
        port_ssl: usize,
    ) -> io::Result<()> {
        if let Ok(mut server) = IDENTITY_SERVER_INNER.lock() {
            if server.is_none() {
                *server = Some(IdentityServerInner::start(
                    context,
                    tls_password,
                    tls_ca_password,
                    host,
                    port,
                    port_ssl,
                )?);
            }
        }
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        if let Some(server) = IDENTITY_SERVER_INNER
            .lock()
            .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?
            .as_mut()
        {
            server.stop()?;
        }
        Ok(())
    }
}

struct IdentityServerInner {
    process: Child,
}

impl IdentityServerInner {
    fn start(
        context: &Path,
        tls_password: &Path,
        tls_ca_password: &Path,
        host: Ipv4Addr,
        port: usize,
        port_ssl: usize,
    ) -> io::Result<Self> {
        eprintln!("starting server");

        let build = Command::new("docker")
            .args([
                "build",
                // "-q",
                "--secret",
                format!("id=password,src={}", tls_password.to_string_lossy()).as_str(),
                "--secret",
                "id=password-ca,src=password.txt",
                format!("id=password-ca,src={}", tls_ca_password.to_string_lossy()).as_str(),
                context.to_string_lossy().as_ref(),
            ])
            .output()?;
        println!("docker eerror: {}", from_utf8(&build.stderr).unwrap());

        let build = from_utf8(&build.stdout)
            .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;
        println!("build: {}", build);
        Ok(Self {
            process: Command::new("docker")
                .args([
                    "run",
                    "--rm",
                    "--env-file",
                    "password-local.env",
                    // "-p",
                    // format!("{}:{}", host, port).as_str(),
                    // "-p",
                    // format!("{}:{}", host, port_ssl).as_str(),
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
