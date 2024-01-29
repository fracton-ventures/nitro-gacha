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
    abigen!(
        MyToken,
        r#"[
            function mint() external
            function setRandomValue() external
            function getRandomValue(uint256 token_id) external view returns (uint256)
        ]"#
    );

    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let address: Address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".parse()?;

    let wallet = LocalWallet::from_str(
        &"0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
    )?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = SignerMiddleware::new(provider, wallet.clone().with_chain_id(chain_id));
    let client = Arc::new(client);

    let start_time = Instant::now();
    let mut count: u128 = 0;
    let my_token = MyToken::new(address, client);

    for _ in 0..1000 {
        my_token.mint().send().await?;
        count += 1;
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
