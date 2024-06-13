use sha2::{Digest, Sha256};
use std::error::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub fn double_sha256(data: &[u8]) -> Vec<u8> {
    let first_hash = Sha256::digest(data);
    let second_hash = Sha256::digest(&first_hash);
    second_hash.to_vec()
}

pub async fn read_exact_bytes(
    stream: &mut TcpStream,
    buf: &mut [u8],
) -> Result<(), Box<dyn Error>> {
    let mut bytes_read = 0;
    while bytes_read < buf.len() {
        let n = stream
            .read(&mut buf[bytes_read..])
            .await
            .expect("Error reading stream");
        if n == 0 {
            return Err("Stream might have been closed".into());
        }
        bytes_read += n;
    }
    Ok(())
}
