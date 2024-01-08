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
      description: "Generate the data proof for the given `index`",
      params: [
        {
          name: "data_index",
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
    queryAppData: {
      description: "Query app data with a specific app id",
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
      type: "Vec<Vec<u8>>",
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
