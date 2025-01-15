use std::error::Error;

use ethers::prelude::*;
use ethers::signers::LocalWallet;
use ethers::utils::keccak256;
use hex;

pub async fn generate_signature_proof(
    message: &str,
    private_keys: Vec<&str>,
) -> Result<String, Box<dyn Error>> {
    let mut rs_array = Vec::new();
    let mut ss_array = Vec::new();
    let mut v_array = Vec::new();

    let message_bytes = message.as_bytes();

    let message_hash = keccak256(message_bytes);

    for private_key in private_keys {
        let wallet: LocalWallet = private_key.parse()?;

        // sign message hash
        let signature = wallet.sign_hash(H256::from(message_hash))?;

        // extract r, s, v
        let mut r_bytes = [0u8; 32];
        let mut s_bytes = [0u8; 32];
        signature.r.to_big_endian(&mut r_bytes);
        signature.s.to_big_endian(&mut s_bytes);
        let r = hex::encode(r_bytes);
        let s = hex::encode(s_bytes);
        let v = if signature.v == 27 { 0u8 } else { 1u8 };

        rs_array.push(r);
        ss_array.push(s);
        v_array.push(v);
    }

    // encode to signatureProof format
    let signature_proof = ethers::abi::encode(&[
        ethers::abi::Token::Array(
            rs_array.iter().map(|r| ethers::abi::Token::FixedBytes(hex::decode(r).unwrap())).collect()
        ),
        ethers::abi::Token::Array(
            ss_array.iter().map(|s| ethers::abi::Token::FixedBytes(hex::decode(s).unwrap())).collect()
        ),
        ethers::abi::Token::Array(
            v_array.iter().map(|&v| ethers::abi::Token::Uint(ethers::types::U256::from(v))).collect()
        ),
    ]);

    let signature_proof_hex = format!("0x{}", hex::encode(signature_proof));

    Ok(signature_proof_hex)
}