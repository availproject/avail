import {createApi} from "./api";
import {Keyring} from "@polkadot/api";
import config from "./config";

/**
 * Example of transferring tokens from Alice to Bob.
 */
async function main() {
    // instantiate the api
    const api = await createApi()
    // construct the keyring after the api
    const keyring = new Keyring({type: 'sr25519'});
    // add sender to our keyring with a hard-derivation path (empty phrase, so uses dev)
    const sender = keyring.addFromUri(config.mnemonic);
    // receiver address from config
    const receiver = config.receiver;
    // amount we wesh to send
    const amount = config.amount;
    // transfer some amount to BOB
    const transfer = api.tx.balances.transfer(receiver, amount);
    // sign and send the transaction using our account
    const hash = await transfer.signAndSend(sender);

    console.log('Transfer sent with hash', hash.toHex());
}

main()
    .catch(console.error)
    .finally(() => process.exit());
