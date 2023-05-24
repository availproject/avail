import {createApi} from "./api";

/**
 *
 */
async function main() {
    const api:any = await createApi()

    const hashBlock = "0x693ae169131a736a88c672b313a5abbf97e7e2dc0d2a4c47a220874453260c10";
    // query proof for the row and col in the provided block
    const proof = await api.rpc.kate.queryProof([{row: 0, col: 0}], hashBlock);
    console.log(new Uint8Array(proof))

    await api.disconnect();
}


main().catch(console.error);