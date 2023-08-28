import {createApi} from "./api";
import {ISubmittableResult} from "@polkadot/types/types";
import config from "./config";
import {Keyring} from "@polkadot/api";

/**
 * Example of getting the proof for the particular leaf in the block.
 */
async function main() {
    const api: any = await createApi()
    // index of a leaf in the merkle trie for which the proof is generated
    // hash of the block from where to get the proof
    const keyring = new Keyring({type: 'sr25519'});
    const sender = keyring.addFromUri(config.mnemonic);
    let transactionIndex: number | undefined = 0;
    // submit one data transaction and wait until block is finalized
    let res: ISubmittableResult = await new Promise(async (resolve) => {
        api.tx.dataAvailability.submitData("0x01")
            .signAndSend(sender, async (result: ISubmittableResult) => {
                console.log(`Tx status: ${result.status}`);
                if (result.isFinalized) {
                    console.log("Block is finalized.")
                    transactionIndex = result.txIndex;
                    resolve(result);
                }
            });
    });

    // after block finalization we can query for the Merkle proof of the data submitted
    const hashBlock = res.status.asFinalized;

    const daHeader = await api.rpc.kate.queryDataProof(transactionIndex, hashBlock);
    console.log(`Fetched proof from Avail for txn index ${transactionIndex} inside block ${hashBlock}`);
    console.log(`Root: ${daHeader.root}`);
    console.log(`Proof: ${daHeader.proof}`);
    console.log(`NumberOfLeaves: ${daHeader.numberOfLeaves}`);
    console.log(`Leaf_index: ${daHeader.leaf_index}`);
    console.log(`Leaf: ${daHeader.leaf}`);
    return daHeader;
}

main()
    .catch(console.error)
    .finally(() => process.exit());
