import {createApi} from "./api";
import {Keyring} from "@polkadot/api";
import config from "./config";
import {ISubmittableResult} from "@polkadot/types/types";

/**
 * Example of getting the proof for data submission.
 */
async function main() {
    const api: any = await createApi();

    const keyring = new Keyring({type: 'sr25519'});
    const sender = keyring.addFromUri(config.mnemonic);
    console.log("Sending transaction...")
    // submit one transaction and wait until block is finalized
    let res: ISubmittableResult = await new Promise(async (resolve) => {
        api.tx.dataAvailability.submitData("0x02")
            .signAndSend(sender, async (result: ISubmittableResult) => {
                console.log(`Tx status: ${result.status}`);
                if (result.isFinalized) {
                    console.log("Transaction in finalized block.")
                    resolve(result);
                }
            });
    });

    // after block finalization we can get block hash and query proof
    const hashBlock = res.status.asFinalized;
    // query proof for the row and col in the provided block
    const proof = await api.rpc.kate.queryProof([{row: 0, col: 0}], hashBlock);
    console.log(new Uint8Array(proof))

    await api.disconnect();
}


main().catch(console.error);
