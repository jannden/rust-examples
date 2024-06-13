extern crate base58;
extern crate rand;
extern crate ripemd;
extern crate sha2;

use k256::ecdsa::{SigningKey, VerifyingKey};
use rand::Rng;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

const VERSION_BYTE: u8 = 0x00; // Version byte for Bitcoin mainnet addresses

fn generate_private_key() -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let mut private_key = [0u8; 32];
    rng.fill(&mut private_key);
    private_key
}

fn generate_public_key(private_key: &[u8; 32]) -> Vec<u8> {
    let signing_key =
        SigningKey::from_bytes(private_key.into()).expect("32 bytes, within curve order");
    let verifying_key = VerifyingKey::from(&signing_key);
    let public_key = verifying_key.to_encoded_point(false).to_bytes(); // false for uncompressed key
    public_key.to_vec()
}

fn hash_public_key(public_key: &[u8]) -> Vec<u8> {
    let sha256_hash = Sha256::digest(public_key);
    let ripemd160_hash = Ripemd160::digest(&sha256_hash);
    ripemd160_hash.to_vec()
}

fn generate_address(public_key_hash: &[u8]) -> String {
    let mut address = vec![VERSION_BYTE];
    address.extend(public_key_hash);

    let checksum = double_sha256(&address)[..4].to_vec();
    address.extend(&checksum);

    base58::ToBase58::to_base58(address.as_slice())
}

fn double_sha256(data: &[u8]) -> Vec<u8> {
    let first_hash = Sha256::digest(data);
    let second_hash = Sha256::digest(&first_hash);
    second_hash.to_vec()
}

fn main() {
    let private_key = generate_private_key();
    let public_key = generate_public_key(&private_key);
    let public_key_hash = hash_public_key(&public_key);
    let address = generate_address(&public_key_hash);
    println!("Bitcoin Address: {}", address);
}
