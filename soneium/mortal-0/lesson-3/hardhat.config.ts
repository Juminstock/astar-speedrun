import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: "0.8.24",
  defaultNetwork: "minato",
  networks: {
    minato: {
      url: "https://rpc.minato.soneium.org/",
      chainId: 1946,
      accounts: ["5424a115d4a6eb41a30cdddb635decdd2cfee1b813b45870910aa2bc5dd18e41"]
    }
  }
};

export default config;