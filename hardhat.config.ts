import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  defaultNetwork: "Nitro",
  networks: {
    hardhat: {
      blockGasLimit: 100000000,
    },
    Nitro: {
      url: "http://localhost:8545",
    },
  },
  solidity: "0.8.20",
};

export default config;
