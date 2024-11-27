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

  dataSubmissionsCount(): number {
    return dataSubmissionsCount(this.signedBlock)
  }

  dataSubmissionsAll(): DataSubmission[] {
    return dataSubmissionsAll(this.signedBlock)
  }

  dataSubmissionsBySigner(signer: string): DataSubmission[] {
    return dataSubmissionsBySigner(this.signedBlock, signer)
  }

  dataSubmissionsByIndex(txIndex: number): Result<DataSubmission, string> {
    return dataSubmissionsByIndex(this.signedBlock, txIndex)
  }

  dataSubmissionsByHash(txHash: H256): Result<DataSubmission, string> {
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

export function dataSubmissionsCount(block: SignedBlock): number {
  return dataSubmissionsAll(block).length
}

export function dataSubmissionsAll(block: SignedBlock): DataSubmission[] {
  const dataSubmissions = []
  const txs = block.block.extrinsics
  for (let i = 0; i < txs.length; i += 1) {
    const maybeDataSubmission = dataSubmissionsByIndex(block, i)
    if (maybeDataSubmission.isOk()) {
      dataSubmissions.push(maybeDataSubmission.value)
    }
  }

  return dataSubmissions
}

export function dataSubmissionsBySigner(block: SignedBlock, signer: string): DataSubmission[] {
  return dataSubmissionsAll(block).filter((sub) => {
    return sub.txSigner == signer
  })
}

export function dataSubmissionsByHash(block: SignedBlock, txHash: H256): Result<DataSubmission, string> {
  const index = block.block.extrinsics.findIndex((tx) => {
    return tx.hash.toHex() == txHash.toHex()
  })

  if (index == -1) return err("Failed to extract data. Transaction has not been found")

  return dataSubmissionsByIndex(block, index)
}

export function dataSubmissionsByIndex(block: SignedBlock, txIndex: number): Result<DataSubmission, string> {
  const transactions = block.block.extrinsics
  if (txIndex >= transactions.length) return err("Failed to extract data. Transaction has not been found")

  const tx = block.block.extrinsics[txIndex]
  return extractDataSubmissionFromTx(tx, txIndex)
}

export function dataSubmissionsByAppId(block: SignedBlock, appId: number): DataSubmission[] {
  return dataSubmissionsAll(block).filter((sub) => {
    return sub.appId == appId
  })
}

export function extractDataSubmissionDataFromTx(tx: GenericExtrinsic): Result<string, string> {
  const {
    method: { args, method, section },
  } = tx
  if (!isTransactionDataSubmission(tx)) {
    return err("Failed to extract data. The transaction is not of type Data Submit")
  }

  let dataHex = args.map((a) => a.toString()).join(", ")
  if (dataHex.startsWith("0x")) {
    dataHex = dataHex.slice(2)
  }

  return ok(dataHex)
}

export function extractDataSubmissionFromTx(tx: GenericExtrinsic, txIndex: number): Result<DataSubmission, string> {
  const maybeData = extractDataSubmissionDataFromTx(tx)
  if (maybeData.isErr()) {
    return err(maybeData.error)
  }

  const txHash = tx.hash
  const data = maybeData.value
  const txSigner = tx.signer.toString()
  const appId = extractAppIdFromTx(tx)
  return ok(new DataSubmission(txHash, txIndex, data, txSigner, appId))
}

export function extractAppIdFromTx(tx: GenericExtrinsic): number {
  return parseInt((tx as any).__internal__raw.signature.appId.toString())
}

export function isTransactionDataSubmission(tx: GenericExtrinsic): boolean {
  const {
    method: { method, section },
  } = tx
  return section == "dataAvailability" && method == "submitData"
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
    return extractDataSubmissionFromTx(tx, txIndex).unwrapOr(null)
  }

  toAscii(): string {
    return fromHexToAscii(this.hexData)
  }
}
