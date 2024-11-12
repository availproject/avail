import { ApiPromise } from "@polkadot/api"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { EventRecord, H256, SignedBlock } from "@polkadot/types/interfaces/types"
import { err, ok, Result } from "neverthrow"
import { SubmittableExtrinsic } from "@polkadot/api/types"
import { Block, BN, KeyringPair } from ".."
import { parseTransactionResult } from "../utils"
import { GenericExtrinsic } from "@polkadot/types"

export enum WaitFor {
  BlockInclusion,
  BlockFinalization,
}

export async function getBlockHashAndTxHash(
  result: ISubmittableResult,
  api: ApiPromise,
): Promise<[H256, number, H256, number]> {
  const txHash = result.txHash as H256
  const txIndex: number = result.txIndex || 22
  let blockHash = txHash

  if (result.status.isFinalized) {
    blockHash = result.status.asFinalized as H256
  } else {
    blockHash = result.status.asInBlock as H256
  }

  const header = await api.rpc.chain.getHeader(blockHash)
  const blockNumber: number = header.number.toNumber()

  return [txHash, txIndex, blockHash, blockNumber]
}

export async function signAndSendTransaction(
  tx: SubmittableExtrinsic<"promise">,
  account: KeyringPair,
  waitFor: WaitFor,
  options?: TransactionOptions,
): Promise<Result<ISubmittableResult, string>> {
  const optionWrapper = options || {}

  return await new Promise<Result<ISubmittableResult, string>>((res, _) => {
    tx.signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
      if (result.isError || (result.isInBlock && waitFor == WaitFor.BlockInclusion) || result.isFinalized) {
        res(ok(result))
      }
    }).catch((reason) => {
      res(err(reason))
    })
  })
}

export async function signAndSendAndParseTransaction(
  api: ApiPromise,
  tx: SubmittableExtrinsic<"promise">,
  account: KeyringPair,
  waitFor: WaitFor,
  options?: TransactionOptions,
): Promise<Result<TxResultDetails, TransactionFailed>> {
  const maybeTxResult = await signAndSendTransaction(tx, account, waitFor, options)
  if (maybeTxResult.isErr()) return err(new TransactionFailed(maybeTxResult.error, null))

  const maybeParsed = await parseTransactionResult(api, maybeTxResult.value)
  if (maybeParsed.isErr()) return err(new TransactionFailed(maybeParsed.error.reason, maybeParsed.error.details))

  return ok(maybeParsed.value)
}

export type GenericFailure = { isErr: true; reason: string }

export interface TransactionOptions {
  app_id?: number
  nonce?: number
  tip?: BN
  era?: number
  blockHash?: H256
}

export class TransactionFailed {
  constructor(
    public reason: string,
    public details: TxResultDetails | null,
  ) {}
}

export class TxResultDetails {
  constructor(
    public txResult: ISubmittableResult,
    public events: EventRecord[],
    public txHash: H256,
    public txIndex: number,
    public blockHash: H256,
    public blockNumber: number,
  ) {}

  async fetchBlock(api: ApiPromise): Promise<Block> {
    return await Block.New(api, this.blockHash)
  }

  async fetchGenericTransaction(api: ApiPromise): Promise<GenericExtrinsic> {
    const block = await Block.New(api, this.blockHash)
    const tx = block.transactionByIndex(this.txIndex)._unsafeUnwrap()
    return tx
  }
}

export class FailedTxResult {
  constructor(
    public reason: string,
    public details: TxResultDetails | null,
  ) {}
}

export interface MultisigTimepoint {
  height: number
  index: number
}
