import {createApi} from "./api";

/**
 * Example of getting the proof for the particular leaf in the block.
 */
async function main() {
    const api: any = await createApi()
    // index of a leaf in the merkle trie for which the proof is generated
    const dataIndex = 0;
    // hash of the block from where to get the proof
    const hashBlock = "0x693ae169131a736a88c672b313a5abbf97e7e2dc0d2a4c47a220874453260c10";

    const daHeader = await api.rpc.kate.queryDataProof(dataIndex, hashBlock);
    console.log(`Fetched proof from Avail for txn index ${dataIndex} inside block ${hashBlock}`);
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