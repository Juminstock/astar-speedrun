# Simple project on Astar using Hardhat

This project aims to explain how to build and deploy smart contracts on Astar Network locally using Hardhat. We'll create a simple project and I'll explain how to configure it to deploy on Astar EVM and Astar zkEVM.

## 1. Install Hardhat

Hardhat is used through a local installation in your project. To install it, you need to create an npm project by going to an empty folder. Hardhat projects are ```Node.js``` projects with the hardhat package installed and a ```hardhat.config.js``` file.

1.1 First, create a folder and npm project:

```bash
mkdir hardhat-project
cd hardhat-project
```
```bash
npm init
```

1.2 Then, you need to install Hardhat:

```bash
npm install --save-dev hardhat
```

You can check if Hardhat has been installed correctly by running this command:

```bash
npx hardhat --version
```

You should see an output similar to this:

![Hardhat version output](/public/hardhat-version.png)

## 2. Create Hardhat project

2.1 Once Hardhat is installed, you can create a Hardhat project. Run this command to interact with the CLI and create it: 

```bash
npx hardhat init 
```

You should see an output similar to this:

![Hardhat CLI output](/public/hardhat-init-output.png)

> [!NOTE]
> If you want to see more information about HardHat and this process, check it out the [Official Hardhat Documentation ↗](https://hardhat.org/hardhat-runner/docs/getting-started#overview).

## 3. Create smart contracts on Hardhat project

A ```contracts``` folder has been created in the root of your project, inside this folder you will see a ```Lock.sol``` contract, you can use this template without problem. I also provide you with another template about a ```Rocket contract```, feel free to use either one.

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
## 4. Configure Hardhat project to use Astar Network

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

> [!NOTE]
> If you're using JavaScript, [follow this Official Hardhat Guide ↗](https://hardhat.org/hardhat-runner/docs/config).

> [!NOTE]
> If you want to use Astar EVM, just change the RPC. Check it out the [Official Astar Documentation ↗](https://docs.astar.network/docs/build/environment/endpoints#public-endpoints).

## 5. Deploy smart contracts on Astar Network

5.1 We're ready to deploy our smart contract on Astar zkEVM or Astar EVM! We will have to add some more, but we're almost ready. First, we need to add ```hardhat-ignition``` plugin with this command:

```bash
npm install --save-dev @nomicfoundation/hardhat-ignition-ethers
```

5.2 Then, we will need to create a module to deploy this contract. Inside the ```ignition``` folder in the root of your project, create a folder called ```modules``` and inside create a ```Rocket.ts``` file. Lastly, add this code to this file:

```typescript
import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

export default buildModule("Apollo", (m) => {
  const apollo = m.contract("Rocket", ["Saturn V"]);

  m.call(apollo, "launch", []);

  return { apollo };
});
```

5.3 Compile your contract:

```bash
npx hardhat compile
```

5.4 Deploy your contract: 

```bash
npx hardhat ignition deploy ignition/modules/Rocket.ts --network zkyoto
```

You should see an output similar to this:

![Deploy output with Hardhat](/public/deploy-zkyoto.png)

5.5 As you can see, a contract address has been created, copy it and paste into [Astar zkEVM (or EVM) explorer ↗](https://astar-zkevm.explorer.startale.com/).
