use secp256k1::{PublicKey, SecretKey};
use sha3::{Digest, Sha3_256};
use pyo3::prelude::*;
use rlp::RlpStream;

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_addr, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_contract_address, m)?)?;
    Ok(())
}

#[pyfunction]
fn generate_addr(private_key: String) -> PyResult<String> {
    let secp = secp256k1::Secp256k1::new();
    let mut private_key_vec = Vec::new();
    let mut private_iterator = private_key.split(':');
    loop {
        if let Some(byte) = private_iterator.next(){
            private_key_vec.push(u8::from_str_radix(byte, 16).unwrap_or(0));
        }
        else{
            break;
        }
    }

    let private_key = SecretKey::from_slice(&private_key_vec).unwrap();

    let public_key = PublicKey::from_secret_key(&secp, &private_key);

    let mut hasher = Sha3_256::new();
    hasher.update(&public_key.serialize_uncompressed());
    let public_key_hash = hasher.finalize();
    let address = format!(
        "0x{}",
        public_key_hash[12..]
            .into_iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    );

    Ok(address)
}

#[pyfunction]
fn calculate_contract_address(sender: &str, sender_nonce: u64) -> PyResult<String> {
    let sender_bytes = hex::decode(&sender[2..]).expect("Invalid sender address hex");
    let encoded = {
        let mut rlp_stream = RlpStream::new();
        rlp_stream.append(&sender_bytes);
        rlp_stream.append(&sender_nonce);
        rlp_stream.out().to_vec()
    };
    let mut hasher = Sha3_256::new();
    hasher.update(encoded);
    let hashed = hasher.finalize();

    let address = format!(
        "0x{}",
        hashed[12..]
            .into_iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    );

    Ok(address)
}