import {createApi} from './api';
import {Header} from '@polkadot/types/interfaces/runtime'

/**
 * Subscribes to new blocks and displays block number every time new block is seen bu the node.
 */
async function main() {
    const api = await createApi();

    let waitForBlocks = 5;
    let count = 0;
    // Subscribe to the new headers, callback is fired when new headers are found
    const unsubscribe = await api.rpc.chain.subscribeNewHeads((header: Header) => {
        console.log(`Chain is at block: #${header.number}`);
        count++;
        if (waitForBlocks === count) {
            console.log(`Unsubscribing from a new headers subscription after ${count} blocks`);
            unsubscribe();
            process.exit(0)
        }
    });
}

main().catch(console.error)