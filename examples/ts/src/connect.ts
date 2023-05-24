import {createApi} from "./api";

/**
 * The following example shows how to connect to a node and display some basic information.
 */
async function main() {
    // Create the API and wait until ready
    const api = await createApi();

    // Retrieve the chain and node information via rpc calls
    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version()
    ]);

    console.log(`Connected to chain ${chain} using ${nodeName} and node version ${nodeVersion}`);
}

main()
    .catch(console.error)
    .finally(() => process.exit());