import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import type { EventRecord, ExtrinsicStatus, H256 } from '@polkadot/types/interfaces';
import type { ISubmittableResult, SignatureOptions } from '@polkadot/types/types';
import yargs from 'yargs/yargs';

const keyring = new Keyring({ type: 'sr25519' });

async function cli_arguments() {
    return yargs(process.argv.slice(2)).options({
        e: {
            description: 'WSS endpoint',
            alias: 'endpoint',
            type: 'string',
            default: 'wss://testnet.polygonavail.net/ws'
        },
        n: {
            description: 'count for subscribing',
            alias: 'count',
            type: 'number',
            default: -1
        }

    }).argv;
}

async function createApi(argv: any): Promise<ApiPromise> {
    const provider = new WsProvider(argv.e);

    // Create the API and wait until ready
    return ApiPromise.create({
        provider,
        types: {
            DataLookup: {
                size: 'u32',
                index: 'Vec<(u32,u32)>'
            },
            KateExtrinsicRoot: {
                hash: 'Hash',
                commitment: 'Vec<u8>',
                rows: 'u16',
                cols: 'u16'
            },
            KateHeader: {
                parentHash: 'Hash',
                number: 'Compact<BlockNumber>',
                stateRoot: 'Hash',
                extrinsicsRoot: 'KateExtrinsicRoot',
                digest: 'Digest',
                appDataLookup: 'DataLookup'
            },
            Header: 'KateHeader',
            AppId: 'u32',
        },
        signedExtensions: {
            CheckAppId: {
                extrinsic: {
                    appId: 'u32'
                },
                payload: {}
            },
        },
    });
}

async function main() {
    const argv = await cli_arguments();
    const api = await createApi(argv);
    const chain = await api.rpc.system.chain();
    console.log("connected to chain: " + chain.toString());
    const metadata = await api.rpc.state.getMetadata();
    let count = 0;
    // Subscribe to the new headers
    const unsubHeads = await api.rpc.chain.subscribeNewHeads((lastHeader) => {
        console.log(`${chain}: last block #${lastHeader.number} has hash ${lastHeader.hash}`);

        if (++count === argv.n && argv.n !== -1) {
            unsubHeads();
            process.exit(0);
        }
    });

}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});