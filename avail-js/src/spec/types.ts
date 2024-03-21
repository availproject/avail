export const types = {
  AppId: "Compact<u32>",
  DataLookupIndexItem: {
    appId: "AppId",
    start: "Compact<u32>",
  },
  DataLookup: {
    size: "Compact<u32>",
    index: "Vec<DataLookupIndexItem>",
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
  DataProof: {
    roots: "TxDataRoots",
    proof: "Vec<H256>",
    numberOfLeaves: "Compact<u32>",
    leafIndex: "Compact<u32>",
    leaf: "H256",
  },
  TxDataRoots: {
    dataRoot: "H256",
    blobRoot: "H256",
    bridgeRoot: "H256",
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
    data: "Vec<u8>",
    id: "u64",
  },
  Message: {
    _enum: {
      ArbitraryMessage: "ArbitraryMessage",
      FungibleToken: "FungibleToken",
    },
  },
  MessageType: {
    _enum: ["ArbitraryMessage", "FungibleToken"],
  },
  FungibleToken: {
    assetId: "H256",
    amount: "String",
  },
  BoundedData: "Vec<u8>",
  ArbitraryMessage: "BoundedData",
  Cell: {
    row: "u32",
    col: "u32",
  },
}
