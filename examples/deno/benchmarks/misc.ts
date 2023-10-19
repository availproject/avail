import { ApiPromise} from 'https://deno.land/x/polkadot@0.2.42/api/mod.ts';
import { UnsubscribePromise } from 'https://deno.land/x/polkadot@0.2.42/api-base/types/base.ts';

export async function waitForBlockInclusion(api: ApiPromise, blockNumber: number): Promise<string> {
    const lambda = new Promise<UnsubscribePromise>((res, _) => {
        const unsub = api.rpc.chain.subscribeNewHeads((header) => {
            if (header.number.toNumber() >= blockNumber) {
                res(unsub)
            }
        })
    });
    const unsub = await lambda;
    unsub()

    return (await api.rpc.chain.getBlockHash(blockNumber)).toString();
}

export async function waitForBlockFinalization(api: ApiPromise, blockNumber: number): Promise<string> {
    const lambda = new Promise<UnsubscribePromise>((res, _) => {
        const unsub = api.rpc.chain.subscribeFinalizedHeads((header) => {
            if (header.number.toNumber() >= blockNumber) {
                res(unsub)
            }
        })
    })
    const unsub = await lambda;
    unsub()

    return (await api.rpc.chain.getBlockHash(blockNumber)).toString();
}