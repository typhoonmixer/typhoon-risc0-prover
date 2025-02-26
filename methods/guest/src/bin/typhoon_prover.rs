use std::io::Read;

use alloy_sol_types::SolValue;
use alloy_primitives::U256;
use risc0_zkvm::guest::env;
use sha2::{Sha256, Digest};

use num_bigint::{BigUint};
use std::str::FromStr;
use guests::commitment_hasher;
use guests::mimc5Sponge;
use guests::merkle_tree;
use guests::block_verifier;
use std::hash::{BuildHasher, Hasher, RandomState};

alloy_sol_types::sol! {
    struct Input {
        string[] roots;
        string[] nullifierHashs;
        string[] days;
        string recipient;
        string relayer;
        string relayerFee;
        string[] nullifiers;
        string[] secrets;
        string[][] pathElements;
        string[][] pathIndices;
        string changeLeaf;
        string changeSecret;
        string changeNullifier;
        string[] balances;
        string change;
        string amountOut;
        string previousBlockHash;
        string[] nextTreeRootsHashs;
        string[] blocksIndex;
        string[][] blockRootTrees;
        string finalBlockHash;
    }

    struct Output {
        uint256[] publicInputs;
    }

}

fn main() {
    // Read the input data for this application.
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    // Decode and parse the input
    let input: Input = <Input>::abi_decode(&input_bytes, true).unwrap();

    checkInputs(
        input.roots.clone(),
        input.nullifierHashs.clone(),
        input.days.clone(),
        input.nullifiers.clone(),
        input.secrets.clone(),
        input.pathElements.clone(),
        input.pathIndices.clone(),
        input.blockRootTrees.clone(),
        input.balances.clone(),
    );
    for i in 0..input.roots.len() {
        let (commitmentHash, nullifierHash) =
            commitment_hasher::commitment_hash(BigUint::from_str(&input.secrets[i].clone()).unwrap(), BigUint::from_str(&input.nullifiers[i].clone()).unwrap());
        if nullifierHash != BigUint::from_str(&input.nullifierHashs[i].clone()).unwrap() {
            panic!("Invalid nullifierHash")
        }
        let leafHash = mimc5Sponge::MiMC5Sponge(
            [U256::from_str(&commitmentHash.to_string()).unwrap(), U256::from_str(&input.balances[i].clone()).unwrap()],
            U256::from_str("0").unwrap(),
        );
        merkle_tree::merkleTreeChecker(
            10_usize,
            leafHash,
            U256::from_str(&input.roots[i].clone()).unwrap(),
            input.days[i].parse::<u128>().unwrap(),
            input.pathElements[i].iter().map(|e| U256::from_str(e).unwrap()).collect(),
            input.pathIndices[i].iter().map(|e| e.parse::<u8>().unwrap()).collect(),
        );
    }

    block_verifier::verifier(
        input.roots.iter().map(|e| U256::from_str(e).unwrap()).collect(),
        U256::from_str(&input.previousBlockHash).unwrap(),
        input.nextTreeRootsHashs.iter().map(|e| U256::from_str(e).unwrap()).collect(),
        input.blockRootTrees.iter().map(|e| e.iter().map(|f| U256::from_str(f).unwrap()).collect()).collect(),
        U256::from_str(&input.finalBlockHash).unwrap(),
        &mut input.blocksIndex.iter().map(|e| e.parse::<usize>().unwrap()).collect(),
    );

    let mut totalBalance: BigUint = BigUint::from_str("0").unwrap();

    for i in 0..input.balances.len() {
        totalBalance += BigUint::from_str(&input.balances[i].clone()).unwrap();
    }

    if BigUint::from_str(&input.amountOut.clone()).unwrap() > totalBalance {
        panic!("Insuficient balance!")
    }

    let (changeCommitment, _) = commitment_hasher::commitment_hash(BigUint::from_str(&input.changeSecret.clone()).unwrap(), BigUint::from_str(&input.changeNullifier.clone()).unwrap());
    let finalLeaf = mimc5Sponge::MiMC5Sponge([U256::from_str(&changeCommitment.to_string()).unwrap(), U256::from_str(&input.change.clone()).unwrap()], U256::from_str("0").unwrap());

    if finalLeaf != U256::from_str(&input.changeLeaf.clone()).unwrap() {
        panic!("Invalid change leaf!")
    }


    // add fake nullifiers to hide the real nullifiers and the number of real nullifiers
    let mut hiddenNullifiers: Vec<U256> = Vec::new();
    for i in 0..input.nullifierHashs.len() {
        let random_int: usize = input.blocksIndex[i].parse::<usize>().unwrap() % 10;
        hiddenNullifiers.push(U256::from_str(&input.nullifierHashs[i].clone()).unwrap());
        let mut new_nullifier = U256::from_str(&input.nullifierHashs[i].clone()).unwrap();
        for j in 0..random_int {
            let mut chasher = Sha256::new();

            chasher.update(new_nullifier.to_string());
            let aux_new_nullifier = BigUint::from_bytes_le(&chasher.finalize());
            new_nullifier = U256::from_str(&aux_new_nullifier.to_string()).unwrap();
            hiddenNullifiers.push(new_nullifier.clone());
        }
    }
    shuffle(&mut hiddenNullifiers);
    let mut output: Vec<U256> = [U256::from_str(&input.recipient).unwrap(), U256::from_str(&input.relayer).unwrap(), U256::from_str(&input.relayerFee).unwrap(), U256::from_str(&input.amountOut).unwrap(), U256::from_str(&input.changeLeaf).unwrap(), U256::from_str(&input.finalBlockHash).unwrap()].to_vec();
    let out = Output { publicInputs : [output, hiddenNullifiers].concat()};
    
    //output = [output, hiddenNullifiers].concat();
    env::commit_slice(&out.abi_encode());

}

pub fn shuffle<BigUint>(vec: &mut [BigUint]) {
    let n: usize = vec.len();
    for i in 0..(n - 1) {
        // Generate random index j, such that: i <= j < n
        // The remainder (`%`) after division is always less than the divisor.
        let j = (rand() as usize) % (n - i) + i;
        vec.swap(i, j);
    }
}

pub fn rand() -> u64 {
    RandomState::new().build_hasher().finish()
}

fn checkInputs(
    roots: Vec<String>,
    nullifierHashs: Vec<String>,
    days: Vec<String>,
    nullifiers: Vec<String>,
    secrets: Vec<String>,
    pathElements: Vec<Vec<String>>,
    pathIndices: Vec<Vec<String>>,
    blockRootTrees: Vec<Vec<String>>,
    balances: Vec<String>
) { 
    if roots.len() != nullifierHashs.len() {
        panic!("wrong number of nullifier hashs")
    }
    if roots.len() != days.len() {
        panic!("wrong number of days")
    }
    if roots.len() != nullifiers.len() {
        panic!("wrong number of nullifiers")
    }
    if roots.len() != secrets.len() {
        panic!("wrong number of days")
    }
    if roots.len() != pathElements.len() {
        panic!("wrong number of path elements")
    }
    if roots.len() != pathIndices.len() {
        panic!("wrong number of path indices")
    }
    if roots.len() != blockRootTrees.len() {
        panic!("wrong number of root trees")
    }
    if roots.len() != balances.len() {
        panic!("wrong number of balances")
    }
}
