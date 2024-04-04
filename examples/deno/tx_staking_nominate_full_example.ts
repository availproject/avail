/// TODO DOC
///
///

import { ApiPromise, Keyring, WsProvider} from "https://deno.land/x/polkadot@0.2.42/api/mod.ts";
import { BN } from "https://deno.land/x/polkadot@0.2.42/util/mod.ts";
import { H256 } from "https://deno.land/x/polkadot@0.2.42/types/interfaces/types.ts";
import { API_EXTENSIONS, API_RPC, API_TYPES } from "./api_options.ts";
import { ISubmittableResult } from "https://deno.land/x/polkadot@0.2.42/types/types/extrinsic.ts";

const api = await ApiPromise.create({
  provider: new WsProvider("ws://127.0.0.1:9944"),
  rpc: API_RPC,
  types: API_TYPES,
  signedExtensions: API_EXTENSIONS,
});

// TODO
const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob");
// TODO
const min_nominator_bond = (await api.query.staking.minNominatorBond()).toString();
const bond_amount = new BN(min_nominator_bond == "0" ? "1000000000000000000000" : min_nominator_bond);
// TODO
const targets = ["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"];

const staking_bond = api.tx.staking.bond(bond_amount, "Staked");
const staking_nominate = api.tx.staking.nominate(targets);

// Transaction call
const tx_result = await new Promise<ISubmittableResult>((res, _) => {
  api.tx.utility.batchAll([staking_bond, staking_nominate]).signAndSend(account, (result: ISubmittableResult) => {
      console.log(`Tx status: ${result.status}`);
      if (result.isFinalized || result.isError) {
        res(result);
      }
    },
  );
});
console.log(`Tx Hash: ${tx_result.txHash as H256}, Block Hash: ${tx_result.status.asFinalized as H256}`);

// Error handling
const error = tx_result.dispatchError;
if (tx_result.isError) {
  console.log(`Transaction was not executed`);
} else if (error != undefined) {
    if (error.isModule) {
      // for module errors, we have the section indexed, lookup
      const decoded = api.registry.findMetaError(error.asModule);
      const { docs, name, section } = decoded;
      console.log(`${section}.${name}: ${docs.join(' ')}`);
    } else {
      // Other, CannotLookup, BadOrigin, no extra info
      console.log(error.toString());
    }
}

Deno.exit(0);
