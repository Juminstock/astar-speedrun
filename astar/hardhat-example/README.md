# Simple project on Astar using Hardhat

This project aims to explain how to build and deploy smart contracts on Astar Network locally using Hardhat. We'll create a simple project and I'll explain how to configure it to deploy on Astar EVM y Astar zkEVM.

## Install Hardhat

Hardhat is used through a local installation in your project. To install it, you need to create an npm project by going to an empty folder. Hardhat projects are ```Node.js``` projects with the hardhat package installed and a ```hardhat.config.js``` file.

First, create a folder and npm project:

```bash
mkdir hardhat-project
cd hardhat-project
```
```bash
npm init
```

Then, you need to install Hardhat:

```bash
npm install --save-dev hardhat
```

You can check if Hardhat has been installed correctly by running this command:

```bash
npx hardhat --version
```

You should see an output similar to this:

![Hardhat version output](/astar/hardhat-example/public/hardhat-v-output.png)

## Create Hardhat project

Once Hardhat is installed, you can create a Hardhat project. Run this command to interact with the CLI and create it: 

```bash
npx hardhat init 
```

You should see an output similar to this:

![Hardhat CLI output](/astar/hardhat-example/public/hardhat-cli.png)

> [!NOTE]
> If you want to see more information about HardHat and this process, check it out the [Official Hardhat Documentation ↗](https://hardhat.org/hardhat-runner/docs/getting-started#overview).

## Create smart contracts on Hardhat project

A ```contracts``` folder has been created in the root of your project, inside this folder you will see a ```Lock.sol``` contract, you can use this template without problem. I also provide you with another template about a Rocket contract, feel free to use either one.

### Rocket contract
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract Rocket {
    string public name;
    string public status;

    constructor(string memory _name) {
        name = _name;
        status = "ignition";
    }

    function launch() public {
        status = "lift-off";
    }
}
```
## Configure Hardhat project to use Astar Network

Once the project and the smart contract are created, we'll need to configure the network to deploy and its RPC. Come on to configure the project to deploy on Astar EVM or Astar zkEVM.

For do that, we will have to go to ```hardhat.config.ts```, this file normally lives in the root of your project. The entirety of your hardhat setup (your config, plugins and custom tasks) is contained in this file.

So, if you're using TypeScript, the ```hardhat.config.ts``` file will look like this:

```typescript
import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: "0.8.24",
};

export default config;
```

> [!NOTE]
> If you're using JavaScript, [follow this Official Hardhat Guide](https://hardhat.org/hardhat-runner/docs/config).

Just copy and paste this in your ```hardhat.config.ts``` file:

```typescript
import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: "0.8.24",
  defaultNetwork: "zkyoto",
  networks: {
    zkyoto: {
      url: "https://rpc.startale.com/zkyoto",
      chainId: 6038361,
      accounts: ["paste-here-your-private-key"]
    }
  }
};

export default config;
```

## Deploy smart contracts on Astar Network