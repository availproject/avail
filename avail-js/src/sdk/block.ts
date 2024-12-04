import { ApiPromise } from "@polkadot/api"
import { GenericExtrinsic } from "@polkadot/types"
import { H256, SignedBlock } from "@polkadot/types/interfaces/runtime"
import { fromHexToAscii } from "./utils"
import { EventRecord, Events } from "."

export class Block {
  signedBlock: SignedBlock

  static async New(api: ApiPromise, blockHash: H256): Promise<Block> {
    const block = await api.rpc.chain.getBlock(blockHash)
    return new Block(block)
  }

  static async NewBestBlock(api: ApiPromise): Promise<Block> {
    const blockHash = await api.rpc.chain.getBlockHash()
    return Block.New(api, blockHash)
  }

  static async NewFinalizedBlock(api: ApiPromise): Promise<Block> {
    const blockHash = await api.rpc.chain.getFinalizedHead()
    return Block.New(api, blockHash)
  }

  constructor(block: SignedBlock) {
    this.signedBlock = block
  }

  async blockHash(api: ApiPromise): Promise<H256> {
    return await api.rpc.chain.getBlockHash(this.signedBlock.block.header.number.toNumber())
  }

  async fetchEvents(api: ApiPromise, txIndex?: number): Promise<EventRecord[]> {
    return Events.fetchEvents(api, await this.blockHash(api), txIndex)
  }

  transactionCount(): number {
    return transactionCount(this.signedBlock)
  }

  transactionAll(): GenericExtrinsic[] {
    return transactionAll(this.signedBlock)
  }

  transactionBySigner(signer: string): GenericExtrinsic[] {
    return transactionBySigner(this.signedBlock, signer)
  }

  transactionByIndex(txIndex: number): GenericExtrinsic | null {
    return transactionByIndex(this.signedBlock, txIndex)
  }

  transactionByHash(txHash: H256): GenericExtrinsic[] {
    return transactionByHash(this.signedBlock, txHash)
  }

  transactionByAppId(appId: number): GenericExtrinsic[] {
    return transactionByAppId(this.signedBlock, appId)
  }

  transactionHashToIndex(txHash: H256): number[] {
    return transactionHashToIndex(this.signedBlock, txHash)
  }

  dataSubmissionsCount(): number {
    return dataSubmissionsCount(this.signedBlock)
  }

  dataSubmissionsAll(): DataSubmission[] {
    return dataSubmissionsAll(this.signedBlock)
  }

  dataSubmissionsBySigner(signer: string): DataSubmission[] {
    return dataSubmissionsBySigner(this.signedBlock, signer)
  }

  dataSubmissionsByIndex(txIndex: number): DataSubmission | null {
    return dataSubmissionsByIndex(this.signedBlock, txIndex)
  }

  dataSubmissionsByHash(txHash: H256): DataSubmission | null {
    return dataSubmissionsByHash(this.signedBlock, txHash)
  }

  dataSubmissionsByAppId(appId: number): DataSubmission[] {
    return dataSubmissionsByAppId(this.signedBlock, appId)
  }
}

export function transactionCount(block: SignedBlock): number {
  return block.block.extrinsics.length
}

export function transactionAll(block: SignedBlock): GenericExtrinsic[] {
  return block.block.extrinsics
}

export function transactionBySigner(block: SignedBlock, signer: string): GenericExtrinsic[] {
  return block.block.extrinsics.filter((tx) => {
    return tx.signer.toString() == signer
  })
}

export function transactionByIndex(block: SignedBlock, txIndex: number): GenericExtrinsic | null {
  const transactions = block.block.extrinsics
  if (txIndex >= transactions.length) return null

  return transactions[txIndex]
}

export function transactionByHash(block: SignedBlock, txHash: H256): GenericExtrinsic[] {
  return block.block.extrinsics.filter((tx) => {
    return tx.hash.toHex() == txHash.toHex()
  })
}

export function transactionByAppId(block: SignedBlock, appId: number): GenericExtrinsic[] {
  return block.block.extrinsics.filter((tx) => {
    return extractAppIdFromTx(tx) == appId
  })
}

export function transactionHashToIndex(block: SignedBlock, txHash: H256): number[] {
  const indices: number[] = []
  for (const [index, tx] of block.block.extrinsics.entries()) {
    if (tx.hash.toHex() == txHash.toHex()) {
      indices.push(index)
    }
  }
  return indices
}

export function dataSubmissionsCount(block: SignedBlock): number {
  return dataSubmissionsAll(block).length
}

export function dataSubmissionsAll(block: SignedBlock): DataSubmission[] {
  const dataSubmissions = []
  const txs = block.block.extrinsics
  for (let i = 0; i < txs.length; i += 1) {
    const ds = dataSubmissionsByIndex(block, i)
    if (ds != null) {
      dataSubmissions.push(ds)
    }
  }

  return dataSubmissions
}

export function dataSubmissionsBySigner(block: SignedBlock, signer: string): DataSubmission[] {
  return dataSubmissionsAll(block).filter((sub) => {
    return sub.txSigner == signer
  })
}

export function dataSubmissionsByHash(block: SignedBlock, txHash: H256): DataSubmission | null {
  const index = block.block.extrinsics.findIndex((tx) => {
    return tx.hash.toHex() == txHash.toHex()
  })

  if (index == -1) return null

  return dataSubmissionsByIndex(block, index)
}

export function dataSubmissionsByIndex(block: SignedBlock, txIndex: number): DataSubmission | null {
  const transactions = block.block.extrinsics
  if (txIndex >= transactions.length) return null

  const tx = block.block.extrinsics[txIndex]
  return extractDataSubmissionFromTx(tx, txIndex)
}

export function dataSubmissionsByAppId(block: SignedBlock, appId: number): DataSubmission[] {
  return dataSubmissionsAll(block).filter((sub) => {
    return sub.appId == appId
  })
}

export function extractDataSubmissionDataFromTx(tx: GenericExtrinsic): string | null {
  if (tx.method.section != "dataAvailability" && tx.method.method != "submitData") {
    return null
  }

  let dataHex = tx.method.args.map((a) => a.toString()).join(", ")
  if (dataHex.startsWith("0x")) {
    dataHex = dataHex.slice(2)
  }

  return dataHex
}

export function extractDataSubmissionFromTx(tx: GenericExtrinsic, txIndex: number): DataSubmission | null {
  const data = extractDataSubmissionDataFromTx(tx)
  if (data == null) return null

  const txHash = tx.hash
  const txSigner = tx.signer.toString()
  const appId = extractAppIdFromTx(tx)
  return new DataSubmission(txHash, txIndex, data, txSigner, appId)
}

export function extractAppIdFromTx(tx: GenericExtrinsic): number {
  return parseInt((tx as any).__internal__raw.signature.appId.toString())
}

export class DataSubmission {
  constructor(
    public txHash: H256,
    public txIndex: number,
    public hexData: string,
    public txSigner: string,
    public appId: number,
  ) {}

  static fromGenericTx(tx: GenericExtrinsic, txIndex: number): DataSubmission | null {
    return extractDataSubmissionFromTx(tx, txIndex)
  }

  toAscii(): string {
    return fromHexToAscii(this.hexData)
  }
}
