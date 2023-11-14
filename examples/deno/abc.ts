import { ApiPromise, WsProvider } from 'https://deno.land/x/polkadot@0.2.42/api/mod.ts';
import { API_RPC, API_TYPES, API_EXTENSIONS } from './api_options.ts'
import { UnsubscribePromise } from 'https://deno.land/x/polkadot@0.2.42/api-base/types/base.ts';

const api = await ApiPromise.create({ provider: new WsProvider("wss://couscous-devnet.avail.tools/ws"), rpc: API_RPC, types: API_TYPES, signedExtensions: API_EXTENSIONS  });




// wss://goldberg.avail.tools/ws
// ws://127.0.0.1:9944
const block_target = (await api.rpc.chain.getHeader()).number.toNumber() + 2;

const readHeaders = new Promise<UnsubscribePromise>((res, _) => {
    const unsub = api.rpc.chain.subscribeNewHeads((header) => {
        console.log(`Chain is at block: #${header.number}`);
        let slot  = api.query.babe.currentSlot().then((s) => {
            console.log(s.toHuman());
        });
    })
})
//console.log((await api.query.system.digest()).toHuman());

const unsub = await readHeaders;
unsub();
Deno.exit(0);
