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
    // Construct the keyring after the API
    const keyring = new Keyring({type: 'sr25519'});
    const sender = keyring.addFromUri(config.mnemonic);
    // submit one data transaction and wait until block is finalized
    // in order to dispatch data root once the block is finalized
    let res: ISubmittableResult = await new Promise(async (resolve) => {
        api.tx.dataAvailability.submitData("0x01")
            .signAndSend(sender, async (result: ISubmittableResult) => {
                console.log(`Tx status: ${result.status}`);
                if (result.isFinalized) {
                    console.log("Block is finalized.")
                    resolve(result);
                }
            });
    });

    // destination domain always 1000
    const destinationDomain = 1000;
    // data availability bridge router address deployed on Sepolia network e.g. 0x000000000000000000000000aAB16A9fb03D5845193e87F596Fa610FCE6054F0
    const bridgeRouterEthAddress = "0x000000000000000000000000bD824890A51ed8bda53F51F27303b14EFfEbC152";
    // hash of the block to dispatch data root
    const blockHash = res.status.asFinalized
    const header = await api.rpc.chain.getHeader(blockHash);

    const tx = api.tx.nomadDABridge.tryDispatchDataRoot(destinationDomain, bridgeRouterEthAddress, header);

    await dispatchDataRoot(api, sender, tx)
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
                            console.log(`Data root dispatched. Block: ${result.status.asFinalized} finalized.`);
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
