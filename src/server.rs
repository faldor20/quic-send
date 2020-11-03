use std::{sync::Arc, error::Error, fs, io, net::{IpAddr, Ipv4Addr, SocketAddr}, path::PathBuf};
use rcgen;
use futures::TryFutureExt;
use tokio::{self, stream::StreamExt};
use tracing::{Instrument, error, info, info_span};
use super::common;
use quinn::{self, Certificate, CertificateChain, Endpoint, Incoming, PrivateKey, ServerConfig, ServerConfigBuilder, TransportConfig};
use quinn::Read;

/// Constructs a QUIC endpoint configured to listen for incoming connections on a certain address
/// and port.
///
/// ## Returns
///
/// - a sream of incoming QUIC connections
/// - server certificate serialized into DER format
#[allow(unused)]
pub fn make_server_endpoint(bind_addr: SocketAddr) -> Result<(Incoming, Vec<u8>), Box<dyn Error>> {
    let (server_config, server_cert) = configure_server()?;
    let mut endpoint_builder = Endpoint::builder();
    endpoint_builder.listen(server_config);
    let (_endpoint, incoming) = endpoint_builder.bind(&bind_addr)?;
    Ok((incoming, server_cert))
}

// Returns default server configuration along with its certificate.
fn configure_server() -> Result<(ServerConfig, Vec<u8>), Box<dyn Error>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = cert.serialize_der().unwrap();
    let priv_key = cert.serialize_private_key_der();
    let priv_key = PrivateKey::from_der(&priv_key)?;

    let mut transport_config = TransportConfig::default();
    //transport_config.stream_window_uni(0).unwrap();
    let mut server_config = ServerConfig::default();
    server_config.transport = Arc::new(transport_config);
    let mut cfg_builder = ServerConfigBuilder::new(server_config);
    let cert = Certificate::from_der(&cert_der)?;
    cfg_builder.certificate(CertificateChain::from_certs(vec![cert]), priv_key)?;

    Ok((cfg_builder.build(), cert_der))
}

/// Runs a QUIC server bound to localhost and port 5500
pub async fn run_server() {
    let addr:SocketAddr= "[::1]:4433".parse().expect("");
    let (mut incoming, _server_cert) = make_server_endpoint(addr).unwrap();
    // accept a single connection
    let incoming_conn = incoming.next().await.unwrap();
    let new_conn = incoming_conn.await.unwrap();
    println!(
        "[server] connection accepted: addr={}",
        new_conn.connection.remote_address()
    );
    
   // let recv=(new_conn.uni_streams.next() .await.unwrap().expect("couldn't find recv st"));
    
    //handle_request(recv);
}
async fn handle_connection( conn:quinn::Connecting ) -> Result<(),io::Error> {
    let quinn::NewConnection{connection, mut uni_streams,..}=conn.await?;
    async{
        info!("connection made");

        // Each stream initiated by the client constitutes a new request.
        while let Some(mut stream) = uni_streams.next().await {
            let mut stream = match stream {
                Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
                    info!("connection closed");
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
                Ok(s) => s,
            };
            
            tokio::spawn(
                handle_request(  stream)
                    .unwrap_or_else(move |e| error!("failed: {reason}", reason = e.to_string()))
                    .instrument(info_span!("request")),
            );
        }
        Ok(())
    };
    Ok(())
    
}
async fn handle_request( stream:  quinn::RecvStream) -> Result<(),io::Error> {
    let mut file = fs::File::create("./out").expect("file not working");
    //stream.read_to_end()
    //common::write_stream_to_file( stream, &mut file);
    
    
    
    Ok(())

}