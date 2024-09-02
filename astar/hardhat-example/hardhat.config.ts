import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: "0.8.26",
  defaultNetwork: "zkyoto",
  networks: {
    zkyoto: {
      url: "https://rpc.startale.com/zkyoto",
      chainId: 6038361,
      accounts: ["dad5ef83ebd73e643bab14cab9bbfe928528121426cef0a174021adb5ee3200a"]
    }
  }
};

export default config;