use std::{env, net::{IpAddr, Ipv4Addr, SocketAddr}};
mod client;
mod server;
mod common;
use quinn; 
fn main() {
    let args:Vec<String>=env::args().collect();
    match args.len() {
        1=>{
            println!("use either -s 'filePath' to start a send server or -r 'ip' to request a file from a server ")
        },
        3|4=>{
            match  args[1].as_str() {
            "-s"=> sendServer_file(&args[2]),
            "-r"=>request_file(&args[2],&args[3]),
            _=>println!("sorry i don't know what that is")
        }
        },
        _=>{
            println!("to many args")
        

        }
    }
    
    
    /* new_socket.send(buf) */

}
/* 
fn request_file(args_1: &String, args_2: &String) -> () {
    todo!()
}

fn sendServer_file(clientIP: &String) -> () {
    println!("Hello, world!");
    
  //  let con=qp2p::QuicP2p::new()?;
    let peer=con.connect_to();
    
}


use std::net::{IpAddr, Ipv4Addr, SocketAddr};

async fn client( addr:String) -> Result<(), Error> {

    let mut config = Config::default();
    config.ip = Some(IpAddr::V4(Ipv4Addr::LOCALHOST));
    let mut quic_p2p = QuicP2p::with_config(Some(config.clone()), Default::default(), true)?;
    let serveraddr=addr.parse().expect("failed parsing server ip");

    let (peer_2, connection) = quic_p2p.connect_to(&serveraddr).await?;
    Ok(())
} */

/* use anyhow::{anyhow, bail, Context, Result};
use futures::{StreamExt, TryFutureExt};
use structopt::{self, StructOpt};*/

//use tracing_futures::Instrument as _; 



