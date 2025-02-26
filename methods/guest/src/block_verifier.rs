use num_bigint::BigUint;
use std::str::FromStr;

use crate::mimc5Sponge::MiMC5Sponge;
use alloy_primitives::U256;

pub fn verifier(
    roots: Vec<U256>,
    prevBlockHash: U256,
    nextBlocksRoots: Vec<U256>,
    curBlockTreesRoots: Vec<Vec<U256>>,
    finalBlockHash: U256,
    blockIndex: &mut Vec<usize>,
) {
    let mut curBlockIndex = blockIndex[0].clone();
    let mut curIndex: usize = 0;
    let mut prevHash: U256 = prevBlockHash.clone();

    // validate roots
    for i in 0..roots.len() {
        if !curBlockTreesRoots[i].contains(&roots[i].clone()) {
            panic!("Invalid root for block!");
        }
    }

    blockIndex.dedup();

    for i in 0..nextBlocksRoots.len(){

        if(curBlockIndex == blockIndex[curIndex]){
            let mut rootHashAux: U256 = curBlockTreesRoots[curIndex][0].clone();

            for j in 1..curBlockTreesRoots[curIndex].len(){
                rootHashAux = MiMC5Sponge([rootHashAux.clone(), curBlockTreesRoots[curIndex][j].clone()], U256::from_str("0").unwrap())
            }
            prevHash = MiMC5Sponge([prevHash ,rootHashAux.clone()], U256::from_str("0").unwrap());

            match blockIndex.get(curIndex+1) {
                Some(_) => curIndex += 1,
                None => continue,
            }
        } else {
            prevHash = MiMC5Sponge([prevHash ,nextBlocksRoots[i].clone()], U256::from_str("0").unwrap());
        }
        curBlockIndex += 1;
    }

    if finalBlockHash != MiMC5Sponge([prevHash ,nextBlocksRoots[nextBlocksRoots.len() -1].clone()], U256::from_str("0").unwrap()) {
        panic!("Invalid final hash!");
    }
}
