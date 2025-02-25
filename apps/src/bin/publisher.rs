// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This application demonstrates how to send an off-chain proof request
// to the Bonsai proving service and publish the received proofs directly
// to your deployed app contract.

use alloy::{
    network::EthereumWallet, providers::ProviderBuilder, signers::local::PrivateKeySigner,
    sol_types::SolValue,
};
use alloy_primitives::{Address, U256};
use anyhow::{Context, Result};
use clap::Parser;
use risc0_ethereum_contracts::encode_seal;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use url::Url;

use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::error::Error;

use std::time::Duration;

use bonsai_sdk::blocking::Client;
use methods::{TYPHOON_PROVER_ELF, TYPHOON_PROVER_ID};
use risc0_zkvm::{compute_image_id, serde::to_vec, Receipt};



#[derive(Serialize)]
struct Metadata{
    image_id: String,
    version: String,
}

fn main() -> Result<()> {
    let client = Client::from_env(risc0_zkvm::VERSION)?;
   
    // Compute the image_id, then upload the ELF with the image_id as its key.
    let image_id = hex::encode(compute_image_id(TYPHOON_PROVER_ELF)?);
    client.upload_img(&image_id, TYPHOON_PROVER_ELF.to_vec())?;

    let md = Metadata{
        image_id: image_id,
        version: risc0_zkvm::VERSION.to_string()
    };

    let json_str = serde_json::to_string_pretty(&md).unwrap();
    
    let mut file = File::create("metadata.json").unwrap();

    file.write_all(json_str.as_bytes()).unwrap();


    Ok(())
}
