export const API_RPC = {
  kate: {
    queryRows: {
      description: "",
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
      type: "Vec<GRow>",
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
      type: "Vec<GDataProof>",
    },
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
  },
};

export const API_TYPES = {
  AppId: "Compact<u32>",
  DataLookupItem: {
    appId: "AppId",
    start: "Compact<u32>",
  },
  CompactDataLookup: {
    size: "Compact<u32>",
    index: "Vec<DataLookupItem>",
  },
  KateCommitment: {
    rows: "Compact<u16>",
    cols: "Compact<u16>",
    commitment: "Vec<u8>",
    dataRoot: "H256",
  },
  V3HeaderExtension: {
    appLookup: "CompactDataLookup",
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
    parentHash: "Hash",
    number: "Compact<BlockNumber>",
    stateRoot: "Hash",
    extrinsicsRoot: "Hash",
    digest: "Digest",
    extension: "HeaderExtension",
  },
  Header: "DaHeader",
  CheckAppIdExtra: {
    appId: "AppId",
  },
  CheckAppIdTypes: {},
  CheckAppId: {
    extra: "CheckAppIdExtra",
    types: "CheckAppIdTypes",
  },
  BlockLengthColumns: "Compact<u32>",
  BlockLengthRows: "Compact<u32>",
  BlockLength: {
    max: "PerDispatchClass",
    cols: "BlockLengthColumns",
    rows: "BlockLengthRows",
    chunkSize: "Compact<u32>",
  },
  PerDispatchClass: {
    normal: "u32",
    operational: "u32",
    mandatory: "u32",
  },
  TxDataRoots: {
    dataRoot: "H256",
    blobRoot: "H256",
    bridgeRoot: "H256",
  },
  DataProof: {
    roots: "TxDataRoots",
    proof: "Vec<H256>",
    numberOfLeaves: "Compact<u32>",
    leafIndex: "Compact<u32>",
    leaf: "H256",
  },
  ProofResponse: {
    dataProof: "DataProof",
    message: "Option<AddressedMessage>",
  },
  AddressedMessage: {
    message: "Message",
    from: "H256",
    to: "H256",
    originDomain: "u32",
    destinationDomain: "u32",
    id: "u64",
  },
  Message: {
    _enum: {
      ArbitraryMessage: "ArbitraryMessage",
      FungibleToken: "FungibleToken",
    },
  },
  FungibleToken: {
    assetId: "H256",
    amount: "u128",
  },
  BoundedData: "Vec<u8>",
  ArbitraryMessage: "BoundedData",
  Cell: {
    row: "u32",
    col: "u32",
  },
  GRawScalar: "U256",
  GProof: "[u8; 48]",
  GRow: "Vec<GRawScalar>",
  GDataProof: "(GRawScalar, GProof)",
};

export const API_EXTENSIONS = {
  CheckAppId: {
    extrinsic: {
      appId: "AppId",
    },
    payload: {},
  },
};
