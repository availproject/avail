import { ApiPromise } from "@polkadot/api"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { EventRecord, H256 } from "@polkadot/types/interfaces/types"
import { err, ok, Result } from "neverthrow"
import { SubmittableExtrinsic } from "@polkadot/api/types"
import { Block, BN, KeyringPair } from ".."
import { parseTransactionResult } from "../utils"
import { GenericEvent, GenericExtrinsic } from "@polkadot/types"
import { Events, CallData } from "./."
import { IEventRecord } from "@polkadot/types/types"

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

  async fetchGenericTransaction(api: ApiPromise): Promise<GenericExtrinsic | null> {
    const block = await Block.New(api, this.blockHash)
    return block.transactionByIndex(this.txIndex)
  }

  findFirstEvent<T>(c: { decode(arg0: EventRecord): T | null }): T | null {
    return Events.findFirstEvent(c, this.events)
  }

  findLastEvent<T>(c: { decode(arg0: EventRecord): T | null }): T | null {
    return Events.findLastEvent(c, this.events)
  }

  findEvent<T>(c: { decode(arg0: EventRecord): T | null }): T[] {
    return Events.findEvent(c, this.events)
  }

  async getData<T>(api: ApiPromise, c: { decode(arg0: GenericExtrinsic): T | null }): Promise<T | null> {
    const tx = await this.fetchGenericTransaction(api)
    return tx ? CallData.getData(tx, c) : null
  }

  checkIfTransactionWasSuccessful(): Boolean {
    return this.findFirstEvent(Events.System.ExtrinsicSuccess) != null
  }

  printDebug() {
    console.log(
      `TxResultDetails {\n  txResult: {...}\n  events: ${this.events.toString()}\n  txHash: ${this.txHash.toHuman()}\n  txIndex: ${this.txIndex.toString()}\n  blockHash: ${this.blockHash.toHuman()}\n  blockNumber: ${this.blockNumber.toString()}\n}`,
    )
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

export class Transaction {
  private api: ApiPromise
  private tx: SubmittableExtrinsic<"promise">

  constructor(api: ApiPromise, tx: SubmittableExtrinsic<"promise">) {
    this.api = api
    this.tx = tx
  }

  async executeWaitForInclusion(
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<TxResultDetails, TransactionFailed>> {
    return await this.execute(WaitFor.BlockInclusion, account, options)
  }

  async executeWaitForFinalization(
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<TxResultDetails, TransactionFailed>> {
    return await this.execute(WaitFor.BlockFinalization, account, options)
  }

  async execute(
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<TxResultDetails, TransactionFailed>> {
    return await signAndSendAndParseTransaction(this.api, this.tx, account, waitFor, options)
  }

  async executeAndForget(account: KeyringPair, options?: TransactionOptions): Promise<H256> {
    const optionWrapper = options || {}
    return await this.tx.signAndSend(account, optionWrapper)
  }

  payment_query_info() {}

  payment_query_fee_details() {}
}
