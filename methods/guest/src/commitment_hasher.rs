use num_bigint::BigUint;

use sha2::{Sha256, Digest};


pub fn commitment_hash(secret: BigUint, nullifier: BigUint) -> (BigUint, BigUint){

    let mut chasher = Sha256::new();

    let mut commitment_str = secret.to_string();
    commitment_str.push_str(&nullifier.to_string());
    chasher.update(commitment_str);
    let commitmentHash = chasher.finalize();

    let mut nhasher = Sha256::new();

    nhasher.update(nullifier.to_string());

    let nullifierHash = nhasher.finalize();

    return (BigUint::from_bytes_le(&commitmentHash), BigUint::from_bytes_le(&nullifierHash))
}