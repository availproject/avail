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

  getSubmitDataAll(): DataSubmission[] {
    return getSubmitDataAll(this.signedBlock)
  }

  getSubmitDataHash(txHash: H256): Result<DataSubmission, string> {
    return getSubmitDataHash(this.signedBlock, txHash)
  }

  getSubmitDataIndex(txIndex: number): Result<DataSubmission, string> {
    return getSubmitDataIndex(this.signedBlock, txIndex)
  }
}

export function getSubmitDataAll(block: SignedBlock): DataSubmission[] {
  const dataSubmissions = []
  const txs = block.block.extrinsics
  for (let i = 0; i < txs.length; i += 1) {
    const maybeDataSubmission = getSubmitDataIndex(block, i)
    if (maybeDataSubmission.isOk()) {
      dataSubmissions.push(maybeDataSubmission.value)
    }
  }

  return dataSubmissions
}

export function getSubmitDataHash(block: SignedBlock, txHash: H256): Result<DataSubmission, string> {
  const index = block.block.extrinsics.findIndex((tx) => {
    return tx.hash.toHex() == txHash.toHex()
  })
  if (index == -1) {
    return err("Failed to extract data. Transaction has not been found")
  }

  return getSubmitDataIndex(block, index)
}

export function getSubmitDataIndex(block: SignedBlock, txIndex: number): Result<DataSubmission, string> {
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
  return ok(new DataSubmission(txHash, txIndex, data, txSigner))
}

function extractDADataFromTx(tx: GenericExtrinsic): Result<string, string> {
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

export class DataSubmission {
  constructor(
    public txHash: H256,
    public txIndex: number,
    public hexData: string,
    public txSigner: string,
  ) {}

  toAscii(): string {
    return fromHexToAscii(this.hexData)
  }
}
