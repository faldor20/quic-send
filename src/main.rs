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
            println!("use either -s 'filePath' to start a send server or -r 'ip' to request a file from a server ")
        },
        3|4=>{
            match  args[1].as_str() {
            "-s"=> server::run_server(),
            "-r"=>client::run_client(&args[2].parse().unwrap(),&args[3]),
            _=>println!("sorry i don't know what that is")
        }
        },
        _=>{
            println!("to many args")
        

        }
    }
    
    
    
  
}


/* async fn main2() -> Result<(), Box<dyn Error>> {
    // server and client are running on the same thread asynchronously
    let addr = "127.0.0.1:5000".parse().unwrap();
    tokio::spawn(run_server(addr));
    run_client(addr).await?;
    Ok(())
}
 */

