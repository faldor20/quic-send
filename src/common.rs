use std::{fs, io::{self, Read, Write}};


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

pub async fn write_file_to_stream(stream:& mut  quinn::SendStream,file: & mut fs::File)->io::Result<u64>{

    let mut buf=[0u8;4096];

    let mut written = 0;
    loop {
        let len = match file.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        stream.write_all(&buf).await?;
        written += len as u64;
    }
}
///writes a file to a stream but reads the entire file into meory ebfore doing so, better for testing pure network speed;
pub async fn write_file_to_stream_simple(stream:& mut  quinn::SendStream,file: & mut fs::File)->io::Result<u64>{
    let mut buf:Vec<u8>=Vec::new();
    let read=file.read_to_end(&mut buf).expect("faield reading file into memroy");
    stream.write_all(&mut buf).await.expect("failed writin file from memory to stream");
    return Ok(read as u64);
}
///writes a file to a stream but reads the entire file into meory ebfore doing so, better for testing pure network speed;
pub async fn write_stream_to_file_simple(stream: quinn::RecvStream,file: & mut fs::File)->io::Result<u64>{
    let mut buf:Vec<u8>=Vec::new();
    let read=stream.read_to_end(1024*1024*1024*2).await.expect("faield reading stream into memroy");
    file.write_all(&mut buf).expect("failed writin stream from memory to file");
    return Ok(read.len() as u64);
}