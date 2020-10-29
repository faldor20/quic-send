use std::{error::Error, fs, io, net::{IpAddr, Ipv4Addr, SocketAddr}, path::PathBuf};
use rcgen;
use futures::TryFutureExt;
use tokio::{self, stream::StreamExt};
use tracing::{Instrument, error, info, info_span};
use super::common;
use quinn;

#[tokio::main]
async fn server()-> Result<(), io::Error>{
    let mut builder= quinn::Endpoint::builder();
    let mut server_config= quinn::ServerConfig::default();
    server_config.transport= std::sync::Arc::new( quinn::TransportConfig::default());
    let mut server_config=quinn::ServerConfigBuilder::new(server_config);
    
    let newConnection=builder.listen(server_config.build());
    let localaddr=SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST),5500);

    let (endpoint, mut incoming)=builder.bind(&localaddr).unwrap();
    while let Some(conn) = incoming.next().await {
        info!("connection incoming");
        tokio::spawn(
            handle_connection(conn).unwrap_or_else(move |e| {
                error!("connection failed: {reason}", reason = e.to_string())
            }),
        );
    }
    
   /*  
    config.ip = Some(IpAddr::V4(Ipv4Addr::LOCALHOST));
    let mut quic_p2p = QuicP2p::with_config(Some(config.clone()), Default::default(), true)?;
    let peer_1 = quic_p2p.new_endpoint()?;
    let con= peer_1.listen()?.next().await?;
    let mut f=std::fs::File::create("test")?;
    
    std::io::copy(&mut con, &mut f);
    let peer1_addr = peer_1.our_endpoint()?; */

    Ok(())
}

async fn handle_connection( conn:quinn::Connecting ) -> Result<(),io::Error> {
    let quinn::NewConnection{connection, mut uni_streams,..}=conn.await?;
    async{
        info!("connection made");

        // Each stream initiated by the client constitutes a new request.
        while let Some(stream) = uni_streams.next().await {
            let stream = match stream {
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
                handle_request( stream)
                    .unwrap_or_else(move |e| error!("failed: {reason}", reason = e.to_string()))
                    .instrument(info_span!("request")),
            );
        }
        Ok(())
    };
    Ok(())
    
}

async fn handle_request(mut stream: quinn::generic::RecvStream<quinn::crypto::rustls::TlsSession>) -> Result<(),io::Error> {
    let mut file = fs::File::create("./out").expect("file not working");
    
    common::write_stream_to_file(&mut stream, &mut file);
    
    
    
    Ok(())

}
///Sorts out certificate stuff
///pass in None for the cert or key and it will generate its own
fn setup_certificate(key_opt:Option<PathBuf>,cert_opt:Option<PathBuf>,server_config: &mut  quinn::ServerConfig)->Result<(),Box< dyn Error>>{
    if let (Some(key_path), Some(cert_path)) = (&key_opt, &cert_opt) {
        let key = fs::read(key_path).expect("failed to read key");
        let key = if key_path.extension().map_or(false, |x| x == "der") {
            quinn::PrivateKey::from_der(&key)?
        } else {
            quinn::PrivateKey::from_pem(&key)?
        };
        let cert_chain = fs::read(cert_path).expect("failed to read certificate chain");
        let cert_chain = if cert_path.extension().map_or(false, |x| x == "der") {
            quinn::CertificateChain::from_certs(quinn::Certificate::from_der(&cert_chain))
        } else {
            quinn::CertificateChain::from_pem(&cert_chain)?
        };
        server_config.certificate(cert_chain, key)?;
    } else {
        let path = std::path::Path::new("./");
        let cert_path = path.join("cert.der");
        let key_path = path.join("key.der");
        let (cert, key) = match fs::read(&cert_path).and_then(|x| Ok((x, fs::read(&key_path)?))) {
            Ok(x) => x,
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                info!("generating self-signed certificate");
                let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
                let key = cert.serialize_private_key_der();
                let cert = cert.serialize_der().unwrap();
                fs::create_dir_all(&path).expect("failed to create certificate directory");
                fs::write(&cert_path, &cert).expect("failed to write certificate");
                fs::write(&key_path, &key).expect("failed to write private key");
                (cert, key)
            }
            Err(e) =>  panic!(e),
            
        };
        let key = quinn::PrivateKey::from_der(&key)?;
        let cert = quinn::Certificate::from_der(&cert)?;
        server_config.certificate(quinn::CertificateChain::from_certs(vec![cert]), key)?;
    }
    Ok(())
}