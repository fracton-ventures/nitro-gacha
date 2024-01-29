// This script does 3 things
// 1. It compiles and deploys the solidity contract
//      Current Problem: I don't have access to docs, and I don't know how to configure the environmental varialbe
// 2. It executes the mint function for NUM_TOKENS times.
// We will use the first private/public key pair provided by Anvil.

use ethers::solc::{Artifact, Project, ProjectPathsConfig};
use ethers::{
    contract::ContractFactory,
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use std::time::Instant;
use std::{convert::TryFrom, str::FromStr};
use std::{path::PathBuf, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    abigen!(
        MyToken,
        r#"[
            function safeMint(address to) external
        ]"#
    );

    // Number of times to mint the NFT
    const NUM_TOKENS: u64 = 10;
    let mut count: u128 = 0;

    // Set up the provider and wallet client
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let wallet = LocalWallet::from_str(
        &"0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
    )?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = SignerMiddleware::new(provider, wallet.clone().with_chain_id(chain_id));
    let client = Arc::new(client);

    // Set up the constructor arguments
    let owner_address: Address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".parse()?;

    // Get the directory for the solidity contract
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sources = root.join("contracts");

    let paths = ProjectPathsConfig::builder()
        .root(&root)
        .sources(&sources)
        .build()
        .unwrap();

    println!("path is: {:?}", paths.sources);

    // Get the solc project instance
    let project = Project::builder()
        .paths(paths)
        .ephemeral()
        .no_artifacts()
        .build()
        .unwrap();

    // Compile hte project and get the artifacts
    let output = project.compile().unwrap();
    let contract = output
        .find_first("MyToken")
        .expect("could not find contract")
        .clone();
    let (abi, bytecode, _) = contract.into_parts();

    // Create a contract factory
    let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());

    // Deploy the contract
    let contract = factory.deploy(owner_address)?.send().await?;

    println!("Contract is: {:?}", contract);

    // Get the contract address and instantiate the contract
    let my_token_address = contract.address();
    let my_token = MyToken::new(my_token_address, client.clone());

    let start_time = Instant::now();

    for _ in 0..NUM_TOKENS {
        my_token.safe_mint(owner_address).send().await?;
        count += 1;
        println!("Successfully minted token number: {}", count);
    }

    let elapsed_time = start_time.elapsed();
    let tps = count * 1_000_000 / elapsed_time.as_micros();

    println!("Elapsed Time: {:?}", elapsed_time);
    println!("TPS: {:?}", tps);

    Ok(())
}
