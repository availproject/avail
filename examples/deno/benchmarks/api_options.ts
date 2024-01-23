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
            type: 'DataProof'
        },
        queryDataProofV2: {
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
            type: 'Vec<Option<Vec<u8>>>',
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
        },
        blockLengthMetrics: {
            description: "Get Block Length",
            params: [
                {
                    name: 'at',
                    type: 'Hash',
                    isOptional: true
                }
            ],
            type: '(BlockLength, u128)'
        },
        queryProofMetrics: {
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
            type: '(Vec<u8>, u128)'
        },
        queryDataProofMetrics: {
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
            type: '(DataProof, u128)'
        },
        queryDataProofV2Metrics: {
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
            type: '(ProofResponse, u128)'
        },
        queryAppDataMetrics: {
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
            type: '(Vec<Option<Vec<u8>>>, u128)',
        },
        queryRowsMetrics: {
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
            type: '(Vec<Vec<u8>>, u128)',
        }
    }
}
