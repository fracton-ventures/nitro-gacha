// This file is the Stylus MyToken Contract

// Stylus programs are webassembly programs deployed onchain, it doesn't use the
// conventional main function as entrypoint. Instead, it uses the #[entrypoint]
// as the entrypoint.
#![cfg_attr(not(feature = "export-abi"), no_main)]
// contract MyToken is ERC721, Ownable {
//     uint256 private token_id;
//     mapping(uint256 => uint256) random_values;

//     constructor(address initialOwner) ERC721("MyToken", "MTK") Ownable(initialOwner) {}

//     function safeMint(address to) public onlyOwner {
//         uint256 token_id = token_id++;
//         _safeMint(to, token_id);
//         setRandomValue(token_id);
//     }

//     function setRandomValue(uint256 token_id) internal {
//         uint256 randomValue = uint256(keccak256(abi.encodePacked(token_id, block.timestamp, msg.sender)));
//         randomValues[token_id] = randomValue;
//     }

//     function getRandomValue(uint256 token_id) public view returns (uint256) {
//         return randomValues[token_id];
//     }
// }
#![cfg_attr(not(feature = "export-abi"), no_std)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod erc721;

use crate::erc721::{ERC721Params, ERC721};
use alloc::{format, string::String, vec::Vec};
use stylus_sdk::{
    alloy_primitives::{keccak256, U256},
    block::timestamp,
    msg,
    prelude::*,
};

// How do we translate the following?
// mapping(uint256 => uint256) randomValues;
pub struct MyTokenParams;
impl ERC721Params for MyTokenParams {
    const NAME: &'static str = "MyToken";
    const SYMBOL: &'static str = "MTK";

    fn token_uri(token_id: U256) -> String {
        format!("uri/{}", token_id)
    }
}

sol_storage! {
    // entrypoint is where the Stylus execution begins. Without which the contract will fail to pass `cargo stylus check`.
    // Withtout the entrypoint attribute, the program cannot be executed onchain.
    #[entrypoint]
    pub struct MyToken {
        #[borrow]
        ERC721<MyTokenParams> erc721;
        uint256 token_id;
        mapping(uint256 => uint256) random_values;
    }
}

// Implement the mint function
#[external]
#[inherit(ERC721<MyTokenParams>)]
impl MyToken {
    pub fn mint(&mut self) -> Result<(), erc721::ERC721Error> {
        let mut token_id = self.token_id.get();
        self.erc721
            .mint(msg::sender(), self.token_id.clone().into())?;
        token_id = token_id + U256::from(1);
        self.token_id.set(token_id);
        Ok(())
    }

    pub fn get_random_value(&self, token_id: U256) -> Result<U256, Vec<u8>> {
        let random_value = self.random_values.get(token_id);
        Ok(U256::from(random_value))
    }
}

impl MyToken {
    pub fn set_random_value(&mut self) -> Result<(), Vec<u8>> {
        let data = format!("{}{}{}", self.token_id.get(), timestamp(), msg::sender());
        let random_value = keccak256(data);
        self.random_values
            .setter(self.token_id.get())
            .set(random_value.into());
        Ok(())
    }
}
