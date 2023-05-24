import {createApi} from "./api";
import {Keyring} from "@polkadot/api";
import config from "./config";

/**
 * Example of transferring tokens from Alice to Bob.
 */
async function main() {
    // instantiate the api
    const api = await createApi()
    // construct the keyring after the API (crypto has an async init)
    const keyring = new Keyring({type: 'sr25519'});
    // add Alice to our keyring with a hard-derivation path (empty phrase, so uses dev)
    const alice = keyring.addFromUri(config.mnemonic);
    // receiver address from config
    const BOB = config.receiver;
    const amount = config.amount;
    // Transfer some amount to BOB
    const transfer = api.tx.balances.transfer(BOB, amount);
    // Sign and send the transaction using our account
    const hash = await transfer.signAndSend(alice);

    console.log('Transfer sent with hash', hash.toHex());
}

main().catch(console.error).finally(() => process.exit());