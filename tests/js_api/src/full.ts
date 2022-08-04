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

        s: {
            description: 'payload to be given in bytes',
            alias: 'payload',
            type: 'number',
            default: 100
        },

        b: {
            description: 'batch size of transactions',
            alias: 'batch',
            type: 'number',
            default: 3
        },

        n: {
            description: 'function name',
            alias: 'function',
            type: 'string',
            default: 'submit_data'
        },

        i: {
            description: 'app id to be given',
            alias: 'app_id',
            type: 'number',
            default: 0
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

interface SignatureOptionsNew extends SignatureOptions {
    app_id: number
}

function randomDigit() {
    return Math.floor(Math.random() * 2);
}

function generateRandomBinary(size: number) {
    let binary = "0x";
    for (let i = 0; i < size; ++i) {
        binary += randomDigit();
    }
    return binary;
}

const generateData = (size: number) => {
    let buffer = Buffer.alloc(size)
    for (let i = 0; i < size; i++) {
        buffer.writeUInt8(Math.floor(Math.random() * 256), i)
    }
    return buffer.toString('hex')
}

//async funtion to get the nonce    
async function getNonce(api: ApiPromise, address: string): Promise<number> {
    const nonce = (await api.rpc.system.accountNextIndex(address)).toNumber();
    return nonce;
}

async function sendTx(api: ApiPromise, sender: KeyringPair, nonce: number, argv: any): Promise<any> {
    try {

        let payload = argv.s;
        let data = generateData(payload);
        let submit = await api.tx.dataAvailability.submitData(data);
        console.log("app id is ", argv.i);
        /* @note here app_id is 1,
        but if you want to have one your own then create one first before initialising here */
        const options: Partial<any> = { app_id: argv.i, nonce: nonce }
        const res = await submit
            .signAndSend(
                sender,  // sender
                options, // options
                (result: ISubmittableResult) => {
                    //uncomment the below line👇 to see the whole status flow of the transaction
                    // console.log(`Tx status: ${result.status}`);
                    if (result.status.isReady) {
                        console.log(`result is ready with nonce ${nonce}`)
                    }
                    if (result.status.isInBlock) {
                        let block_hash = result.status.asInBlock;
                        let extrinsic_hash = result.txHash;
                        console.log(`\nExtrinsic hash: ${result.txHash} with nonce ${nonce} is in block`);
                        // block(block_hash, api);
                        if (argv.n == 'submit_data') {
                            setTimeout(() => {
                                get(api, block_hash, extrinsic_hash);
                            }, 5000);
                        }
                    }
                });
    } catch (e) {
        console.log(e);
        process.exit(1);
    }
}

const sendTxs = async (api: ApiPromise, sender: KeyringPair, nonce: number, argv: any) => {

    const results = [];
    for (let i = 0; i < argv.b; i++) {
        const result = await sendTx(api, sender, nonce, argv)
        results.push(result);
        nonce = nonce + 1
    }
}

//function to retreive data
async function get(api: any, block_hash: H256, extrinsic_hash: H256) {

    const block = await api.rpc.chain.getBlock(block_hash);
    const block_num: number = await block.block.header.number;

    let extrinsics = block.block.extrinsics;

    // console.log("\nretrieving data.....\n ")
    // console.log(`Block Hash: ${block_hash} and extrinsic hash ${extrinsic_hash}\n`);
    // console.log(`Extrinsic data:`);

    let data: Array<string> = [];
    extrinsics.forEach(async (ex: any, index: number) => {
        if (extrinsic_hash == ex.hash.toHex()) {
            console.log(index, ex.toHuman());
            const { method: { args, method, section } } = ex;
            let data_hex = args.map((a: any) => a.toString()).join(', ');
            //data retreived from the extrinsic data
            let str = ''
            for (var n = 0; n < data_hex.length; n += 2) {
                str += String.fromCharCode(parseInt(data_hex.substr(n, 2), 16));
            }
            console.log(`\n 💡 DATA_SUBMITTED : ${str}`);
            console.log(`${section}.${method}(${args.map((a: any) => a.toString()).join(', ')})`);
        }

    });
    process.exit(0);
}


let block = async (hash: H256, api: ApiPromise) => {
    const block = await api.rpc.chain.getBlock(hash);
    const block_num = await block.block.header.number;
    console.log(`💡Tx included in Block Number: ${block_num} with hash ${hash}\n`);
}


async function main() {
    const argv = await cli_arguments();
    const api = await createApi(argv);
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    const metadata = await api.rpc.state.getMetadata();
    let nonce = await getNonce(api, alice.address);
    let non = await getNonce(api, bob.address);
    /*@note: here ALICE test account is used.
    You can use your own account mnemonic using the below code
    // const mnemonic = 'your mneomnic';
    // const acc = keyring.addFromUri(Mnemonic, 'sr25519'); and its address can be used by `acc.address`
    */

    if (argv.n == 'bulk_tx') {
        let tx = await sendTxs(api, alice, nonce, argv);
    }
    else if (argv.n == 'submit_data') {
        let tx = await sendTx(api, alice, nonce, argv)
    }
    else {
        console.log("invalid input");
    }


}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});