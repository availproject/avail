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

export function prepareData(txCount: number): string[] {
    const data: string[] = [];
    
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    const charactersLength = characters.length;
    for (let i = 0; i < txCount; ++i) {
        let array = '';
        let counter = 0;
        while (counter < 16 * 1024) {
            array += characters.charAt(Math.floor(Math.random() * charactersLength));
            counter += 1;
        }
        data.push(array);
    }

    return data;
}
