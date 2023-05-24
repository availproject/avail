import {createApi} from "./api";
import {ApiPromise, Keyring} from "@polkadot/api";
import config from "./config";
import {ISubmittableResult} from "@polkadot/types/types";
import {KeyringPair} from "@polkadot/keyring/types";
import {SubmittableExtrinsic} from "@polkadot/api/promise/types";

/**
 * Example dispatching data root fot the particular block to the destination address and domain.
 */
async function main() {
    // Instantiate the API
    const api = await createApi()
    // Construct the keyring after the API (crypto has an async init)
    const keyring = new Keyring({type: 'sr25519'});
    // Add Alice to our keyring
    const alice = keyring.addFromUri(config.mnemonic);

    // destination domain always 1000
    const destinationDomain = 1000;
    // data availability bridge router address e.g. 0x000000000000000000000000aAB16A9fb03D5845193e87F596Fa610FCE6054F0
    const bridgeRouterEthAddress = "0x00000000000000000000000007AF11e412ed7C343603c0F4b35645f7870686Eb";
    // hash of the block to dispatch data root
    const blockHash = "0x693ae169131a736a88c672b313a5abbf97e7e2dc0d2a4c47a220874453260c10"
    const header = await api.rpc.chain.getHeader(blockHash);

    const tx = api.tx.daBridge.tryDispatchDataRoot(destinationDomain, bridgeRouterEthAddress, header);

    await dispatchDataRoot(api, alice, tx)
}

async function dispatchDataRoot(api: ApiPromise, sender: KeyringPair, dispatchTransaction: SubmittableExtrinsic) {
    return new Promise<void>(async (resolve) => {
        try {
            await dispatchTransaction
                .signAndSend(
                    sender,
                    (result: ISubmittableResult) => {
                        if (result.status.isReady) {
                            console.log(`Txn has been sent to the mempool`);
                        } else if (result.status.isInBlock) {
                            console.log(`Tx hash: ${result.txHash} is in block ${result.status.asInBlock}`);
                        } else if (result.status.isFinalized) {
                            console.log(`Block: ${result.status.asFinalized} finalized.`);
                            resolve();
                        }
                    });
        } catch (e) {
            console.log(e);
            process.exit(1);
        }
    })
}

main()
    .catch((e) => {
        console.error(e);
    })
    .finally(() => {
        process.exit(0);
    });
