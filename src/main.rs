use std::{error::Error, env, net::{IpAddr, Ipv4Addr, SocketAddr}};
mod client;
mod server;
mod common;
mod certificates;
use quinn; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args:Vec<String>=env::args().collect();
    match args.len() {
        1=>{
            println!("use either -s 'host ip' to start a send server or -r 'host ip' 'server socket (ip and port x.x.x.x:xxx)' 'file' to send a file to the server ")
        },
        2|3|4=>{
            match  args[1].as_str() {
            "-s"=> server::run_server(&args[2]).await,
            "-r"=>client::run_client(&args[2],&args[3].parse().unwrap(),&args[4]).await.expect("failed client"),
            _=>println!("sorry i don't know what that is")
        }
        },
        _=>{
            println!("to many args")
        

        }
    }
    
    Ok(())
    
  
}



