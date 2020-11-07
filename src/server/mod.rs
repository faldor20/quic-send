use std::{error::Error, pin::Pin, fs, io, net::{IpAddr, Ipv4Addr, SocketAddr}, path::PathBuf, sync::Arc};
use rcgen;
use futures::TryFutureExt;
use tokio::{self, stream::StreamExt};
use tracing::{Instrument, error, info, info_span};
use self::setup::make_server_endpoint;

use super::common;
use quinn::{self, Certificate, CertificateChain, ConnectionError, Endpoint, Incoming, PrivateKey, ServerConfig, ServerConfigBuilder, TransportConfig};
use quinn::Read;

pub mod setup;

/// Runs a QUIC server bound to localhost and port 5500
pub async fn run_server( self_addr:&String) {
	let addr:SocketAddr= self_addr.parse().expect("couldn't parse socketAddr, rember t add port x.x.x.x:xxxx");
	let (mut incoming, _server_cert) = make_server_endpoint(addr).unwrap();
	// accept a single connection
	
	while let Some(conn)= incoming.next().await {
		tokio::spawn(handle_connection(conn));
/* 		match match conn.await {
			Ok(new_conn)=>{
				println!(
					"[server] connection accepted: addr={}",
					new_conn.connection.remote_address()
				);
				
			},
			Err()=>{
				println!("recieved connection but was unable to finish establishing it")
			},
		}; */
		

	};
	
}
enum Handled_Streams {
	Uni(quinn::IncomingUniStreams),
	Bi(quinn::IncomingBiStreams)
	
}
async fn handle_connection( conn:quinn::Connecting ) -> Result<(),io::Error> {
	let quinn::NewConnection{connection, mut uni_streams,mut bi_streams,..}=conn.await?;
	info!("connection made");
	// Each stream initiated by the client constitutes a new request.
	//probaly
	let a=tokio::spawn(listen_for_bi_streams(bi_streams));
	let b=tokio::spawn(listen_for_uni_streams(  uni_streams));
	a.await?;
	b.await?;
	
	
	Ok(())
	
}

//TODO:i should be able to convert both these methods into one method that is geeric but i don;t really know how
async fn listen_for_uni_streams(uni_streams: quinn::IncomingUniStreams)->Result<(),quinn::ConnectionError>{	 
	let mut in_streams= uni_streams;
	while let Some(stream) =  in_streams.next().await {
		//we have to redclare this as mutable because parameters can't be mutable and if i need to move the var not pass it as a reference
		let  streams = match stream {
			Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
				info!("connection closed");
				return Ok(());
			}
			Err(e) => {
				return Err(e);
			}
			Ok(s) => s,
		};
	
		handle_uni_request( streams).await;
	
	}
	Ok(())
}

async fn listen_for_bi_streams(bi_streams:  quinn::IncomingBiStreams)->Result<(),quinn::ConnectionError>{
	 let mut in_streams=bi_streams;
	while let Some(stream) =  in_streams.next().await {
		
		let  streams = match stream {
			Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
				info!("connection closed");
				return Ok(());
			}
			Err(e) => {
				return Err(e);
			}
			Ok(s) => s,
		};
		
		handle_bi_request( streams).await;
	
	}
	Ok(())
}
///Handle a bi stream 
///The incoming stream will supply a file path.
///Then the file will be sen back in the send stream.
async fn handle_bi_request(bi_stream:(  quinn::SendStream, quinn::RecvStream) ){
	let (mut send,mut recv)=bi_stream;
	let file_path=String::from_utf8( recv.read_to_end(1000).await.expect("couldn't read incoming filepath")).expect("Data that should have been a path couldn't be converted to a string");
	
}

///Handle a unidirectional stream. The stream will be assumed to be an incoming file.
///At a later date i may add the ability for the first incoming byte to be interpreted as a command.
///TODO: make it so the first few bytes is a filename.
async fn handle_uni_request(  uni_stream: quinn::RecvStream)  {
	let mut stream=uni_stream;

	let mut file = fs::File::create("./out").expect("file not working");
	
	//stream.read_to_end()
	stream.rea
	common::write_stream_to_file(  & mut stream, &mut file).await.expect("Failed wirting stream to file.");
	
	/* handle_request(   &mut stream)
	.unwrap_or_else(move |e| error!("failed: {reason}", reason = e.to_string()))
	.instrument(info_span!("request")).await; */
	
	
	

}