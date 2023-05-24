import {createApi} from "./api";

/**
 * Example returning data that belongs to application id.
 */
async function main() {
    const api: any = await createApi()
    const hashBlock = "0x60eaba2542b72a0c17b0e78ad4ae73c3e42e616bb3b5b0dbbc8abfaee5a4aea1";
    const appId = 1;
    const appData = await api.rpc.kate.queryAppData(appId, hashBlock);

    console.log(appData)
    await api.disconnect();
}


main().catch(console.error);