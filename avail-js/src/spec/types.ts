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
  V1HeaderExtension: {
    appLookup: "DataLookup",
    commitment: "KateCommitment",
  },
  V2HeaderExtension: {
    appLookup: "DataLookup",
    commitment: "KateCommitment",
  },
  HeaderExtension: {
    _enum: {
      V1: "V1HeaderExtension",
      V2: "V2HeaderExtension",
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
    root: "H256",
    proof: "Vec<H256>",
    numberOfLeaves: "Compact<u32>",
    leaf_index: "Compact<u32>",
    leaf: "H256",
  },
  DataProofV2: {
    dataRoot: "H256",
    blobRoot: "H256",
    bridgeRoot: "H256",
    proof: "Vec<H256>",
    numberOfLeaves: "Compact<u32>",
    leafIndex: "Compact<u32>",
    leaf: "H256",
  },
  ProofResponse: {
    dataProof: "DataProofV2",
    message: "Option<Message>",
  },
  Message: {
    messageType: "MessageType",
    from: "H256",
    to: "H256",
    originDomain: "u32",
    destinationDomain: "u32",
    data: "Vec<u8>",
    id: "u64",
  },
  MessageType: {
    _enum: ["ArbitraryMessage", "FungibleToken"],
  },
  Cell: {
    row: "u32",
    col: "u32",
  },
}
