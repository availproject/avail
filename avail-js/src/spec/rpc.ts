export const rpc = {
  kate: {
    blockLength: {
      description: "Get Block Length",
      params: [
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        },
      ],
      type: "BlockLength",
    },
    queryProof: {
      description: "Generate the kate proof for the given `cells`",
      params: [
        {
          name: "cells",
          type: "Vec<Cell>",
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        },
      ],
      type: "Vec<u8>",
    },
    queryDataProof: {
      description: "Generate the data proof for the given `transaction_index`",
      params: [
        {
          name: "transaction_index",
          type: "u32",
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        },
      ],
      type: "DataProof",
    },
    queryDataProofV2: {
      description: "Generate the data proof for the given `transaction_index`",
      params: [
        {
          name: "transaction_index",
          type: "u32",
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        },
      ],
      type: "ProofResponse",
    },
    queryAppData: {
      description: "Fetches app data rows for the given app",
      params: [
        {
          name: "app_id",
          type: "AppId",
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        },
      ],
      type: "Vec<Option<Vec<u8>>>",
    },
    queryRows: {
      description: "Query rows based on their indices",
      params: [
        {
          name: "rows",
          type: "Vec<u32>",
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        },
      ],
      type: "Vec<Vec<u8>>",
    },
  },
}
