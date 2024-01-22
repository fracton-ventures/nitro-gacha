import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  defaultNetwork: "localhost",
  networks: {
    hardhat: {
      blockGasLimit: 100000000,
    },
    localhost: {
      url: "http://127.0.0.1:8547",
      accounts: [
        "0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659",
      ],
    },
  },
  solidity: "0.8.20",
};

export default config;
