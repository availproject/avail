export const API_RPC = {
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
            type: 'Vec<(U256, [u8; 48])>'
        },
        queryDataProof: {
            description: 'Generate the data proof for the given `transaction_index`',
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
            type: 'ProofResponse'
        },
        queryAppData: {
            description: 'Fetches app data rows for the given app',
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
            type: 'Vec<Option<Vec<U256>>>',
        },
        queryRows: {
            description: '',
            params: [
                {
                    name: "rows",
                    type: "Vec<u32>"
                },
                {
                    name: "at",
                    type: 'Hash',
                    isOptional: true
                }
            ],
            type: 'Vec<Vec<U256>>',
        }
    }
}

export const API_TYPES = {
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
        rows: "Compact<u16>",
        cols: "Compact<u16>",
        commitment: "Vec<u8>",
        dataRoot: "H256",
    },
    V3HeaderExtension: {
        appLookup: "DataLookup",
        commitment: "KateCommitment",
    },
    HeaderExtension: {
        _enum: {
            V1: null,
            V2: null,
            V3: "V3HeaderExtension",
        },
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
    BlockLengthColumns: "Compact<u32>",
    BlockLengthRows: "Compact<u32>",
    BlockLength: {
        max: 'PerDispatchClass',
        cols: 'BlockLengthColumns',
        rows: 'BlockLengthRows',
        chunkSize: 'Compact<u32>'
    },
    PerDispatchClass: {
        normal: 'u32',
        operational: 'u32',
        mandatory: 'u32'
    },
    DataProof: {
        data_root: 'H256',
        blob_root: 'H256',
        bridge_root: 'H256',
        proof: 'Vec<H256>',
        numberOfLeaves: 'Compact<u32>',
        leafIndex: 'Compact<u32>',
        leaf: 'H256'
    },
    ProofResponse: {
        dataProof: 'DataProof',
        message: 'Option<AddressedMessage>'
    },
    AddressedMessage: {
        message: 'Message',
        from: 'H256',
        to: 'H256',
        origin_domain: 'u32',
        destination_domain: 'u32',
        id: 'u64',
    },
    Message: {
        _enum: {
            ArbitraryMessage: 'ArbitraryMessage',
            FungibleToken: 'FungibleToken'
        }
    },
    MessageType: {
        _enum: [
            'ArbitraryMessage',
            'FungibleToken'
        ]
    },
    FungibleToken: {
        asset_id: 'H256',
        amount: 'String'
    },
    BoundedData: 'Vec<u8>',
    ArbitraryMessage: 'BoundedData',
    Cell: {
        row: 'u32',
        col: 'u32',
    }
}

export const API_EXTENSIONS = {
    CheckAppId: {
        extrinsic: {
            appId: 'AppId'
        },
        payload: {}
    },
}
