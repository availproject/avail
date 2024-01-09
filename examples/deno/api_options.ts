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
            type: 'Vec<u8>'
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
        },
        queryDataProofV2: {
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
            type: 'ProofResponse'
        },
        queryAppData: {
            description: '',
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
            type: 'Vec<Vec<u8>>',
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
      KateCommitmentV2: {
        rows: "Compact<u16>",
        cols: "Compact<u16>",
        commitment: "Vec<u8>",
        dataRoot: "H256",
      },
      V1HeaderExtension: {
        appLookup: "DataLookup",
        commitment: "KateCommitment",
      },
      V2HeaderExtension: {
        appLookup: "DataLookup",
        commitment: "KateCommitmentV2",
      },
      HeaderExtension: {
        _enum: {
          V1: "V1HeaderExtension",
          V2: "V2HeaderExtension",
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
        leafIndex: 'Compact<u32>',
        leaf: 'H256'
    },
    DataProofV2: {
        data_root: 'H256',
        blob_root: 'H256',
        bridge_root: 'H256',
        proof: 'Vec<H256>',
        numberOfLeaves: 'Compact<u32>',
        leafIndex: 'Compact<u32>',
        leaf: 'H256'
    },
    ProofResponse: {
        dataProof: 'DataProofV2',
        message: 'Option<Message>'
    },
    Message: {
        message_type: 'MessageType',
        from: 'H256',
        to: 'H256',
        origin_domain: 'u32',
        destination_domain: 'u32',
        data: 'BoundedData',
        id: 'u64', // a global nonce that is incremented with each leaf
    },
    BoundedData: 'Vec<u8>',
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