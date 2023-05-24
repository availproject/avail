import {createApi} from "./api";
import {BlockHash} from "@polkadot/types/interfaces/chain";

/**
 * Example returning data that belongs to application id.
 */
async function main() {
    const api: any = await createApi()
    // get the latest finalized block hash
    const finalizedHead: BlockHash = await api.rpc.chain.getFinalizedHead();
    console.log(`Latest finalized block: ${finalizedHead}`)
    const appId = 1;
    const appData = await api.rpc.kate.queryAppData(appId, finalizedHead);

    console.log(`Application data: ${appData}`)
    await api.disconnect();
}


main()
    .catch(console.error)
