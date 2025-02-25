use std::str::FromStr;

use crate::mimc5Sponge::MiMC5Sponge;

use num_bigint::BigUint;


pub fn merkleTreeChecker(levels: usize, leaf: BigUint, root: BigUint, day: u128, pathElements: Vec<BigUint>, pathIndices: Vec<u8>){
    let leafHash = MiMC5Sponge([leaf, day.into()], BigUint::from_str("0").unwrap());
    let mut hash: BigUint = BigUint::from_str("0").unwrap();
    for i in 0..levels{
        let mut selectors: [BigUint; 2];
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

fn dualMux(ins: [BigUint; 2], s: u8,) -> [BigUint; 2]{
    if(s == 0){
        return ins;
    } else if(s == 1){
        return [ins[1].clone(), ins[0].clone()];
    } else {
        panic!("Value is not 0 neither 1")
    }
}

fn hashLeftRight(left: BigUint, right: BigUint) -> BigUint{
    
    return MiMC5Sponge([left, right], BigUint::from_str("0").unwrap());
}


