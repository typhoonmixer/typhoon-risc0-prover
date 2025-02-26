use std::str::FromStr;

use crate::mimc5Sponge::MiMC5Sponge;

use alloy_primitives::U256;


pub fn merkleTreeChecker(levels: usize, leaf: U256, root: U256, day: u128, pathElements: Vec<U256>, pathIndices: Vec<u8>){
    let leafHash = MiMC5Sponge([leaf, U256::from(day)], U256::from_str("0").unwrap());
    let mut hash: U256 = U256::from_str("0").unwrap();
    for i in 0..levels{
        let mut selectors: [U256; 2];
        if(i == 0){
            selectors = dualMux([leafHash.clone(), pathElements[i].clone()], pathIndices[i]);
        } else {
            selectors = dualMux([hash.clone(), pathElements[i].clone()], pathIndices[i]);
        }
        hash = hashLeftRight(selectors[0].clone(), selectors[1].clone());
    }

    if(hash != root){
        panic!("Invalid root!")
    }
}

fn dualMux(ins: [U256; 2], s: u8,) -> [U256; 2]{
    if(s == 0){
        return ins;
    } else if(s == 1){
        return [ins[1].clone(), ins[0].clone()];
    } else {
        panic!("Value is not 0 neither 1")
    }
}

fn hashLeftRight(left: U256, right: U256) -> U256{
    
    return MiMC5Sponge([left, right], U256::from_str("0").unwrap());
}


