use std::{net::SocketAddr, sync::Arc, error::Error};

use quinn::*;
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
#[allow(unused)]
// Returns default server configuration along with its certificate.
pub fn configure_server() -> Result<(ServerConfig, Vec<u8>), Box<dyn Error>> {
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

	Ok((cfg_builder.build(), cert_der.to_vec()))
}