//! Example on how to interact with a deployed `stylus-hello-world` program using defaults.
//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. The deployed program is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.

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
    abigen!(
        MyToken,
        r#"[
            function mint() external
            function setRandomValue() external
            function getRandomValue(uint256 token_id) external view returns (uint256)
        ]"#
    );

    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let address: Address = "0xC0F1193d1e8475fcBFBBa764ab1c29179F13fDC1".parse()?;

    let wallet = LocalWallet::from_str(
        &"0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
    )?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let start_time = Instant::now();
    let mut count: u64 = 0;
    let my_token = MyToken::new(address, client);
    for _ in 0..100 {
        my_token.mint().send().await?;
        count += 1;
        println!("Successfully minted token number: {}", count);
    }

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    // println!("TPS: {:?}", count / elapsed_time.as_secs());

    // let random_value = my_token.set_random_value().await?;
    // println!("Set the following random value: {:?}", random_value);

    // let random_value = my_token.get_random_value(U256::from(0)).await?;
    // println!("Get the following random value: {:?}", random_value);

    Ok(())
}
