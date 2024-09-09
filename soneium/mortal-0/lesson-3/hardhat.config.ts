import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: "0.8.24",
  defaultNetwork: "minato",
  networks: {
    minato: {
      url: "https://rpc.minato.soneium.org/",
      chainId: 1946,
      accounts: ["your-private-key-here"]
    }
  }
}

export default config;