import { ApiPromise, WsProvider } from 'https://deno.land/x/polkadot@0.2.42/api/mod.ts';
import { API_RPC, API_TYPES, API_EXTENSIONS } from './api_options.ts'
import { UnsubscribePromise } from 'https://deno.land/x/polkadot@0.2.42/api-base/types/base.ts';

const api = await ApiPromise.create({ provider: new WsProvider("ws://127.0.0.1:9944"), rpc: API_RPC, types: API_TYPES, signedExtensions: API_EXTENSIONS  });

// wss://goldberg.avail.tools/ws
// ws://127.0.0.1:9944
const block_target = (await api.rpc.chain.getHeader()).number.toNumber() + 2;
const readHeaders = new Promise<UnsubscribePromise>((res, _) => {
    const unsub = api.rpc.chain.subscribeNewHeads((header) => {
        console.log(`Chain is at block: #${header.number}`);
/*         if (header.number.toNumber() >= block_target) {
            res(unsub)
        } */
        //console.log(header.digest.toHuman());

        header.digest.logs.forEach((log) => {
            if (!log.isPreRuntime) {
                return;
            }
            let buff = log.asPreRuntime[1];
            let prefix = new DataView(buff.buffer, buff.byteOffset, 1).getUint8(0);
            let validator_index = new DataView(buff.buffer, buff.byteOffset + 1, 4).getUint32(0, true);
            //let slot = new DataView(buff.buffer, buff.byteOffset + 5, 8).getBigUint64(0, true);
            console.log(prefix);
            console.log(validator_index);
            //console.log(slot);
            console.log(log.asPreRuntime[1].toHuman())


/*             let a = log.asPreRuntime;
            let buff = a[1];

            const view = new DataView(buff.buffer, buff.byteOffset, 1);
            let b = view.getUint8(0); // The second argument (true) indicates little-endian byte order.
            console.log(b); */


/*             console.log(a.toHuman()) */

        });
        //console.log(header.digest.logs.toHuman());
    })
})

//console.log((await api.query.system.digest()).toHuman());

const unsub = await readHeaders;
unsub();
Deno.exit(0);
