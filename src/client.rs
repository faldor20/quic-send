use std::{error::Error, fs, io, net::{IpAddr, Ipv4Addr, SocketAddr}, sync::Arc};

use quinn::{ClientConfig, ClientConfigBuilder, Endpoint};

use rustls;
use tracing::{error, info};
use std;


use super::certificates;
pub async fn run_client(server_addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let client_cfg = configure_client();
    let mut endpoint_builder = Endpoint::builder();
    endpoint_builder.default_client_config(client_cfg);

    let (endpoint, _) = endpoint_builder.bind(&"127.0.0.1:0".parse().unwrap())?;

    // connect to server
    let quinn::NewConnection { connection, .. } = endpoint
        .connect(&server_addr, "localhost")
        .unwrap()
        .await
        .unwrap();
    println!("[client] connected: addr={}", connection.remote_address());
    // Dropping handles allows the corresponding objects to automatically shut down
    drop(connection);
    // Make sure the server has a chance to clean up
    endpoint.wait_idle().await;

    Ok(())
}



fn configure_client() -> ClientConfig {
    let mut cfg = ClientConfigBuilder::default().build();
    
    let tls_cfg:&mut rustls::ClientConfig = Arc::get_mut(&mut cfg.crypto).unwrap().into();
    // this is only available when compiled with "dangerous_configuration" feature
    tls_cfg.dangerous().set_certificate_verifier(certificates::SkipServerVerification::new());
    cfg
}