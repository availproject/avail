import { ApiPromise } from "@polkadot/api"
import { GenericExtrinsic } from "@polkadot/types"
import { H256, SignedBlock } from "@polkadot/types/interfaces/runtime"
import { Result, err, ok } from "neverthrow"
import { fromHexToAscii } from "./utils"

export class Block {
  signedBlock: SignedBlock

  static async New(api: ApiPromise, blockHash: H256): Promise<Block> {
    const block = await api.rpc.chain.getBlock(blockHash)
    return new Block(block)
  }

  constructor(block: SignedBlock) {
    this.signedBlock = block
  }

  submitDataCount(): number {
    return submitDataCount(this.signedBlock)
  }

  submitDataAll(): DataSubmission[] {
    return submitDataAll(this.signedBlock)
  }

  submitDataBySigner(signer: string): DataSubmission[] {
    return submitDataBySigner(this.signedBlock, signer)
  }

  submitDataByIndex(txIndex: number): Result<DataSubmission, string> {
    return submitDataByIndex(this.signedBlock, txIndex)
  }

  submitDataByHash(txHash: H256): Result<DataSubmission, string> {
    return submitDataByHash(this.signedBlock, txHash)
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

  transactionByIndex(txIndex: number): Result<GenericExtrinsic, string> {
    return transactionByIndex(this.signedBlock, txIndex)
  }

  transactionByHash(txHash: H256): GenericExtrinsic[] {
    return transactionByHash(this.signedBlock, txHash)
  }
}

export function submitDataCount(block: SignedBlock): number {
  return submitDataAll(block).length
}

export function submitDataAll(block: SignedBlock): DataSubmission[] {
  const dataSubmissions = []
  const txs = block.block.extrinsics
  for (let i = 0; i < txs.length; i += 1) {
    const maybeDataSubmission = submitDataByIndex(block, i)
    if (maybeDataSubmission.isOk()) {
      dataSubmissions.push(maybeDataSubmission.value)
    }
  }

  return dataSubmissions
}

export function submitDataBySigner(block: SignedBlock, signer: string): DataSubmission[] {
  const allSubmissions = submitDataAll(block)

  return allSubmissions.filter((sub) => {
    return sub.txSigner == signer
  })
}

export function submitDataByHash(block: SignedBlock, txHash: H256): Result<DataSubmission, string> {
  const index = block.block.extrinsics.findIndex((tx) => {
    return tx.hash.toHex() == txHash.toHex()
  })
  if (index == -1) {
    return err("Failed to extract data. Transaction has not been found")
  }

  return submitDataByIndex(block, index)
}

export function submitDataByIndex(block: SignedBlock, txIndex: number): Result<DataSubmission, string> {
  const transactions = block.block.extrinsics
  if (transactions.length < txIndex) {
    return err("Failed to extract data. Transaction has not been found")
  }
  const tx = block.block.extrinsics[txIndex]
  const maybeData = extractDADataFromTx(tx)
  if (maybeData.isErr()) {
    return err(maybeData.error)
  }

  const txHash = tx.hash
  const data = maybeData.value
  const txSigner = tx.signer.toString()
  const appId = extractAppIdFromTx(tx)
  return ok(new DataSubmission(txHash, txIndex, data, txSigner, appId))
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

export function transactionByIndex(block: SignedBlock, txIndex: number): Result<GenericExtrinsic, string> {
  const transactions = block.block.extrinsics
  if (transactions.length < txIndex) {
    return err("Failed to find transaction. Index is out of bounds.")
  }

  return ok(transactions[txIndex])
}

export function transactionByHash(block: SignedBlock, txHash: H256): GenericExtrinsic[] {
  return block.block.extrinsics.filter((tx) => {
    return tx.hash.toHex() == txHash.toHex()
  })
}

export function extractDADataFromTx(tx: GenericExtrinsic): Result<string, string> {
  const {
    method: { args, method, section },
  } = tx
  if (section != "dataAvailability" || method != "submitData") {
    return err("Failed to extract data. The transaction is not of type Data Submit")
  }

  let dataHex = args.map((a) => a.toString()).join(", ")
  if (dataHex.startsWith("0x")) {
    dataHex = dataHex.slice(2)
  }

  return ok(dataHex)
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

  toAscii(): string {
    return fromHexToAscii(this.hexData)
  }
}
