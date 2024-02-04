import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import * as dotenv from "dotenv";

dotenv.config({ path: __dirname + "/.env" });
const ACCOUNT_PRIVATE_KEY = process.env.ACCOUNT_PRIVATE_KEY || "";
console.log("PrivateKey set:", !!ACCOUNT_PRIVATE_KEY)

const config: HardhatUserConfig = {
    solidity: "0.8.19",
    paths: {
    artifacts: "./src",
    },
    networks: {
        zKatana: {
        url: `https://rpc.zkatana.gelato.digital`,
        accounts: [ACCOUNT_PRIVATE_KEY]
        },
    },
};

export default config;