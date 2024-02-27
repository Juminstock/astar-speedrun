import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import MinterFactory from "./typedContract/constructors/minter";
import Minter from "./typedContract/contracts/minter";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";

use(chaiAsPromised);

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944");
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" });

describe("minter test", () => {
  let minterFactory: MinterFactory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  
  let contract: Minter;
  const initialState = true;

  before(async function setup(): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider });
    deployer = keyring.addFromUri("//Alice");

    minterFactory = new MinterFactory(api, deployer);

    contract = new Minter(
      (await minterFactory.new(initialState)).address,
      deployer,
      api
    );
  });

  after(async function tearDown() {
    await api.disconnect();
  });
});
