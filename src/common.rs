use std::{fs, io::{self, Write}};


pub async fn write_stream_to_file(stream:& mut quinn::RecvStream,file: &mut fs::File)->io::Result<u64>{
   
    let mut buf=[0;4096];
    let mut written = 0;
    loop {
        let red_res=stream.read(&mut buf ).await;
        let len = match red_res {
            Ok(None) => return Ok(written),
            Ok(Some(len)) => len,
            Err(e) => return Err(e.into()),
        };
        file.write_all(  &buf).expect("failed writing to file");
        written += len as u64;
    }
}