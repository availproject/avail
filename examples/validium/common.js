import {ApiPromise, WsProvider} from "@polkadot/api";

export async function createApi(url) {
    const provider = new WsProvider(url)
    return ApiPromise.create({
        provider,
        rpc: {
            kate: {
                queryAppData: {
                    params: [
                        {
                            name: "app_id",
                            type: "AppId"
                        },
                        {
                            name: "at",
                            type: 'Hash',
                            isOptional: true
                        }
                    ],
                    type: 'Vec<Vec<u8>>',
                },
                queryDataProof: {
                    description: 'Generate the data proof for the given `index`',
                    params: [
                        {
                            name: 'transaction_index',
                            type: 'u32'
                        },
                        {
                            name: 'at',
                            type: 'Hash',
                            isOptional: true
                        }
                    ],
                    type: 'DataProof'
                }, queryProof: {
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

export async function sendTx(api, sender, tx) {
    return new Promise(async (resolve) => {
        try {
            const res = await tx
                .signAndSend(
                    sender,
                    (result) => {
                        if (result.status.isReady) {
                            console.log(`Txn has been sent to the mempool`)
                        }
                        if (result.status.isInBlock) {
                            console.log(`Tx hash: ${result.txHash} is in block ${result.status.asInBlock}`)
                            res()
                            resolve(result)
                        }
                    });

        } catch (e) {
            console.log(e);
            process.exit(1);
        }
    })
}
