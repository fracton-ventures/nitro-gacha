// This script does 3 things
// 1. It compiles and deploys the stylus contract
// 2. It executes the mint function for NUM_TOKENS times.
// We will use the first private/public key pair provided by Anvil.

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Instant;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Number of times to mint the NFT
    const NUM_TOKENS: u64 = 100;

    // Using Nitro Default Endpoint and Prefunded Wallet
    const RPC_ENDPOINT: &str = "http://localhost:8547";
    const PRIVATE_KEY: &str = "0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659";
    const OWNER_ADDRESS: &str = "0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E";
    let mut count: u128 = 0;

    abigen!(
        MyToken,
        r#"[
            function mint() external
        ]"#
    );

    // Using Nitro Default Endpoint and Prefunded Wallet
    let provider = Provider::<Http>::try_from(RPC_ENDPOINT)?;
    let owner_address: Address = OWNER_ADDRESS.parse()?;
    let wallet = LocalWallet::from_str(&PRIVATE_KEY)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = SignerMiddleware::new(provider, wallet.clone().with_chain_id(chain_id));
    let client = Arc::new(client);

    let start_time = Instant::now();
    let my_token = MyToken::new(owner_address, client);

    for _ in 0..NUM_TOKENS {
        count += 1;
        my_token.mint().send().await?;
        println!("Successfully minted token number: {}", count);
    }

    let elapsed_time = start_time.elapsed();
    let tps = count * 1_000_000 / elapsed_time.as_micros();
    println!("Elapsed time: {:?}", elapsed_time);
    println!("TPS: {:?}", tps);

    // let random_value = my_token.set_random_value().await?;
    // println!("Set the following random value: {:?}", random_value);

    // let random_value = my_token.get_random_value(U256::from(0)).await?;
    // println!("Get the following random value: {:?}", random_value);

    Ok(())
}
