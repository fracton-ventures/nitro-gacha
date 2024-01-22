import { ethers } from "hardhat";

const OWNER_ADDRESS = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
const NUM_TOKENS = 1000;

async function main() {
  const MyToken = await ethers.getContractFactory("MyToken");
  const myToken = await MyToken.deploy(OWNER_ADDRESS);
  await myToken.waitForDeployment();

  const startTime = Date.now();
  for (let i = 0; i < NUM_TOKENS; i++) {
    const tx = await myToken.safeMint(OWNER_ADDRESS, { gasLimit: 1000000 });
  }

  const endTime = Date.now();

  const duration = endTime - startTime;
  const tps = NUM_TOKENS / (duration / 1000);
  console.log(`Minted ${NUM_TOKENS} tokens to ${myToken.target}`);
  console.log(`Total time taken: ${duration}ms`);
  console.log(`TPS: ${tps.toFixed(2)}`);
}

function delay(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
