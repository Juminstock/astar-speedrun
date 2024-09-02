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