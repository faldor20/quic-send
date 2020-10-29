use std::{sync::Arc, fs, io, net::{IpAddr, Ipv4Addr, SocketAddr}};

use quinn::{ClientConfig, ClientConfigBuilder};
use rustls::{ServerCertVerified, ServerCertVerifier};
use tracing::{error, info};
use std;
#[tokio::main]
async fn client(server_addr:&SocketAddr,server_name:&str)-> Result<(), io::Error>{
    let mut builder= quinn::Endpoint::builder();
    let address=SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST),0);
    let (endpoint,_)=builder.bind(&address).unwrap();
    //copy certificate stuff here
    endpoint.connect(server_addr, server_name);
Ok(())
}
struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}
impl ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _roots: &rustls::RootCertStore,
        _presented_certs: &[rustls::Certificate],
        _dns_name: webpki::DNSNameRef,
        _ocsp_response: &[u8],
    ) -> Result<ServerCertVerified, rustls::TLSError> {
        Ok(ServerCertVerified::assertion())
    }
}
fn configure_client() -> ClientConfig {
    let mut cfg = ClientConfigBuilder::default().build();
    let tls_cfg: &mut rustls::ClientConfig = std::sync::Arc::get_mut(&mut cfg.crypto).unwrap();
    // this is only available when compiled with "dangerous_configuration" feature
    tls_cfg
        .dangerous()
        .set_certificate_verifier(SkipServerVerification::new());
    cfg
}

fn getCert(ca:& Option<std::path::Path>,client_config:&mut quinn::ClientConfig)->Result<(),io::Error>{
    if let Some(ca_path) = ca {
        client_config
            .add_certificate_authority(quinn::Certificate::from_der(&fs::read(&ca_path)?)?)?;
    } else {
        let dirs = ("./").unwrap();
        match fs::read(dirs.join("cert.der")) {
            Ok(cert) => {
                client_config.add_certificate_authority(quinn::Certificate::from_der(&cert)?)?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                info!("local server certificate not found");
            }
            Err(e) => {
                error!("failed to open local server certificate: {}", e);
            }
        }
    }
    Ok(())
}