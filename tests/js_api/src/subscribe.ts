import config from './config';
import { createApi } from './api';

async function main() {
    const api = await createApi();
    const chain = await api.rpc.system.chain();
    console.log('connected to chain: ' + chain.toString());
    const rep = config.count;
    let count = 0;
    // Subscribe to the new headers
    const unsubHeads = await api.rpc.chain.subscribeNewHeads((lastHeader) => {
        console.log(`${chain}: last block #${lastHeader.number} has hash ${lastHeader.hash}`);

        if (++count === rep && rep !== undefined) {
            unsubHeads();
            process.exit(0);
        }
    });
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
