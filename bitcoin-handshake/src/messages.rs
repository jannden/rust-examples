use bytes::{BufMut, BytesMut};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{utils, MAGIC_BYTES, PROTOCOL_VERSION, USER_AGENT};

pub enum BitcoinMessage {
    Version(Vec<u8>),
    VerAck,
    Ping(u64),
    Pong(u64),
    Other(String, Vec<u8>),
    Empty,
}

pub fn create_version_payload() -> Vec<u8> {
    let mut payload = BytesMut::with_capacity(86);
    payload.put_u32(PROTOCOL_VERSION); // Protocol version
    payload.put_u64(1); // Node services
    payload.put_u64(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    ); // Timestamp
    payload.put_u64(1); // Receiving node services
    payload.put_slice(&[0; 16]); // Receiving node IP
    payload.put_u16(8333); // Receiving node port
    payload.put_u64(1); // Local node services
    payload.put_slice(&[0; 16]); // Local node IP
    payload.put_u16(8333); // Local node port
    payload.put_u64(6484984615390864390); // Nonce
    payload.put_u8(USER_AGENT.as_bytes().len() as u8); // User agent length
    payload.put_slice(USER_AGENT.as_bytes()); // User agent
    payload.put_u32(0); // Start height
    payload.put_u8(1); // Relay

    let checksum = utils::double_sha256(&payload)[..4].to_vec();
    let mut message = BytesMut::with_capacity(24 + payload.len());

    message.put_slice(MAGIC_BYTES); // Magic bytes
    message.put_slice(b"version\0\0\0\0\0"); // Command name padded to 12 bytes
    message.put_u32_le(payload.len() as u32); // Payload length
    message.put_slice(&checksum); // Checksum
    message.put_slice(&payload); // Payload

    message.to_vec()
}

pub fn parse_version_payload(payload: &[u8]) -> Result<(), Box<dyn Error>> {
    if payload.len() < 86 {
        return Err("Invalid payload length for version message".into());
    }

    let protocol_version = u32::from_le_bytes(payload[0..4].try_into()?);
    let services = u64::from_le_bytes(payload[4..12].try_into()?);
    let timestamp = u64::from_le_bytes(payload[12..20].try_into()?);
    let addr_recv_services = u64::from_le_bytes(payload[20..28].try_into()?);
    let addr_recv_ip = &payload[28..44];
    let addr_recv_port = u16::from_le_bytes(payload[44..46].try_into()?);
    let addr_local_services = u64::from_le_bytes(payload[46..54].try_into()?);
    let addr_local_ip = &payload[54..70];
    let addr_local_port = u16::from_le_bytes(payload[70..72].try_into()?);
    let nonce = u64::from_le_bytes(payload[72..80].try_into()?);
    let user_agent_length = payload[80] as usize;
    let user_agent = &payload[81..81 + user_agent_length];
    let start_height =
        u32::from_le_bytes(payload[81 + user_agent_length..85 + user_agent_length].try_into()?);
    let relay = payload[85 + user_agent_length];

    println!("- Protocol Version: {}", protocol_version);
    println!("- Services: {}", services);
    println!("- Timestamp: {}", timestamp);
    println!("- Receiver Address Services: {}", addr_recv_services);
    println!("- Receiver Address IP: {:?}", addr_recv_ip);
    println!("- Receiver Address Port: {}", addr_recv_port);
    println!("- Local Address Services: {}", addr_local_services);
    println!("- Local Address IP: {:?}", addr_local_ip);
    println!("- Local Address Port: {}", addr_local_port);
    println!("- Nonce: {}", nonce);
    println!("- User Agent: {:?}", String::from_utf8_lossy(user_agent));
    println!("- Start Height: {}", start_height);
    println!("- Relay: {}", relay);

    Ok(())
}

pub fn create_getblocks_payload() -> Vec<u8> {
    let mut payload = BytesMut::with_capacity(37);

    payload.put_u32_le(PROTOCOL_VERSION); // Protocol version
    payload.put_u8(1); // Number of block locator hashes (1 for simplicity)
    payload.put_slice(&[0; 32]); // Block locator hash (hash of the last known block)
    payload.put_slice(&[0; 32]); // Hash of the last desired block (set to zero for no limit)

    payload.to_vec()
}
