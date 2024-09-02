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

A ```contracts``` folder has been created in the root of your project, inside this folder you will see a ```Lock.sol``` contract, you can use this template without problem. I also provide you with another template about a simple currency, feel free to use either one.

### Example coin
```solidity
// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.26;

// This is the enter point
contract ExampleCoin {
    address public minter;
    mapping(address => uint) public balances;

    event Sent(address from, address to, uint amount);

    // The constructor only can called once
    constructor() {
        minter = msg.sender;
    }

    // With the requiere, only the minter just to call it
    function mint(address receiver, uint amount) public {
        require(msg.sender == minter);
        balances[receiver] += amount;
    }

    // This is a custom error
    error InsufficientBalance(uint requested, uint available);


    // Finally, this function check the balances and transfer de funds and emit a event
    function send(address receiver, uint amount) public {
        if (amount > balances[msg.sender]) {
            revert InsufficientBalance({requested: amount, available: balances[msg.sender]});
        } else {
            balances[msg.sender] -= amount;
            balances[receiver] += amount;
            emit Sent(msg.sender, receiver, amount);
        }
    }
}
```
## Configure Hardhat project to use Astar Network

Once Hardhat is installed and the project created, we will can dive into the folder structure and understand what Hardhat created for us. The first thing that we have to do is change the network to deploy the contracts.

## Deploy smart contracts on Astar Network