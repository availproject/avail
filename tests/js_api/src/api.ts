import { ApiPromise, WsProvider } from "@polkadot/api";
import config from './config';

export function createApi(): Promise<ApiPromise> {
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