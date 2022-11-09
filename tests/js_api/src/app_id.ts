import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import type { EventRecord, ExtrinsicStatus, H256 } from '@polkadot/types/interfaces';
import type { ISubmittableResult, SignatureOptions } from '@polkadot/types/types';
import yargs from 'yargs/yargs';
import config from './config';

const keyring = new Keyring({ type: 'sr25519' });

async function cli_arguments() {
    return yargs(process.argv.slice(2)).options({
        i: {
            description: 'app id to be given',
            alias: 'app_id',
            type: 'string',
            default: '1'
        }
        
    }).argv;
}

async function createApi(): Promise<ApiPromise> {
    const provider = new WsProvider(config.ApiURL);

    // Create the API and wait until ready
    return ApiPromise.create({
        provider,
        rpc: {
            kate: {
                blockLength: {
                    description: "Get Block Length",
                    params: [
                        {
                            name: 'at',
                            type: 'Hash',
                            isOptional: true
                        }
                    ],
                    type: 'BlockLength'
                },
                queryProof: {
                    description: 'Generate the kate proof for the given `cells`',
                    params: [
                        {
                            name: 'cells',
                            type: 'Vec<Cell>'
                        },
                        {
                            name: 'at',
                            type: 'Hash',
                            isOptional: true
                        },
                    ],
                    type: 'Vec<u8>'
                },
                queryDataProof: {
                    description: 'Generate the data proof for the given `index`',
                    params: [
                        {
                            name: 'data_index',
                            type: 'u32'
                        },
                        {
                            name: 'at',
                            type: 'Hash',
                            isOptional: true
                        }
                    ],
                    type: 'DataProof'
                }
            }
        },
        types: {
            AppId: 'Compact<u32>',
            DataLookupIndexItem: {
                appId: 'AppId',
                start: 'Compact<u32>'
            },
            DataLookup: {
                size: 'Compact<u32>',
                index: 'Vec<DataLookupIndexItem>'
            },
            KateCommitment: {
                rows: 'Compact<u16>',
                cols: 'Compact<u16>',
                dataRoot: 'H256',
                commitment: 'Vec<u8>'
            },
            V1HeaderExtension: {
                commitment: 'KateCommitment',
                appLookup: 'DataLookup'
            },
            VTHeaderExtension: {
                newField: 'Vec<u8>',
                commitment: 'KateCommitment',
                appLookup: 'DataLookup'
            },
            HeaderExtension: {
                _enum: {
                    V1: 'V1HeaderExtension',
                    VTest: 'VTHeaderExtension'
                }
            },
            DaHeader: {
                parentHash: 'Hash',
                number: 'Compact<BlockNumber>',
                stateRoot: 'Hash',
                extrinsicsRoot: 'Hash',
                digest: 'Digest',
                extension: 'HeaderExtension'
            },
            Header: 'DaHeader',
            CheckAppIdExtra: {
                appId: 'AppId'
            },
            CheckAppIdTypes: {},
            CheckAppId: {
                extra: 'CheckAppIdExtra',
                types: 'CheckAppIdTypes'
            },
            BlockLength: {
                max: 'PerDispatchClass',
                cols: 'Compact<u32>',
                rows: 'Compact<u32>',
                chunkSize: 'Compact<u32>'
            },
            PerDispatchClass: {
                normal: 'u32',
                operational: 'u32',
                mandatory: 'u32'
            },
            DataProof: {
                root: 'H256',
                proof: 'Vec<H256>',
                numberOfLeaves: 'Compact<u32>',
                leaf_index: 'Compact<u32>',
                leaf: 'H256'
            },
            Cell: {
                row: 'u32',
                col: 'u32',
            }
        },
        signedExtensions: {
            CheckAppId: {
                extrinsic: {
                    appId: 'AppId'
                },
                payload: {}
            },
        },
    });
}

interface SignatureOptionsNew extends SignatureOptions {
    app_id: number
}



//async funtion to get the nonce    
async function getNonce(api: ApiPromise, address: string): Promise<number> {
    const nonce = (await api.rpc.system.accountNextIndex(address)).toNumber();
    return nonce;
}



async function createKey(api: ApiPromise, sender: KeyringPair, nonce: number, id:string): Promise<any> {
    try {
        /* @note here app_id is 1,
        but if you want to have one your own then create one first before initialising here */
        const options: Partial<any> = { app_id: 0, nonce: nonce }
        const res = await api.tx.dataAvailability.createApplicationKey(id)
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
                        process.exit(0);
                    }
                });
    } catch (e) {
        console.log(e);
        process.exit(1);
    }
}



//function to retreive data



let block = async (hash: H256, api: ApiPromise) => {
    const block = await api.rpc.chain.getBlock(hash);
    const block_num = await block.block.header.number;
    console.log(`💡Tx included in Block Number: ${block_num} with hash ${hash}\n`);
}


async function main() {
    const argv = await cli_arguments();
    const api = await createApi();
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    const metadata = await api.rpc.state.getMetadata();
    // let nonce = await getNonce(api, alice.address);
    let non = await getNonce(api, bob.address);
    /*@note: here BOB test account is used.
    You can use your own account mnemonic using the below code
    // const mnemonic = 'your mneomnic';
    // const acc = keyring.addFromUri(Mnemonic, 'sr25519'); and its address can be used by `acc.address`
    */
    let key = await createKey(api, bob, non, argv.i);


}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});