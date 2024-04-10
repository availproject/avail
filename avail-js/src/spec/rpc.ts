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
      type: "Vec<(U256, [u8; 48])>",
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
      type: "ProofResponse",
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
      type: "Vec<Vec<U256>>",
    },
  },
}
