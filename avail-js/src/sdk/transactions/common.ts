import { ApiPromise } from "@polkadot/api"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { H256 } from "@polkadot/types/interfaces/types"
import { ok, Result } from "neverthrow"
import { TxResultDetails } from "../utils"

export enum WaitFor {
  BlockInclusion,
  BlockFinalization,
}

export function standardCallback(
  result_in: ISubmittableResult,
  result_out: (value: Result<ISubmittableResult, string>) => void,
  waitFor: WaitFor,
) {
  if (result_in.isError) {
    result_out(ok(result_in))
  }

  if (result_in.isInBlock && waitFor == WaitFor.BlockInclusion) {
    result_out(ok(result_in))
  }

  if (result_in.isFinalized) {
    result_out(ok(result_in))
  }
}

export async function getBlockHashAndTxHash(
  result: ISubmittableResult,
  waitFor: WaitFor,
  api: ApiPromise,
): Promise<[H256, number, H256, number]> {
  const txHash = result.txHash as H256
  const txIndex: number = result.txIndex || 22
  let blockHash = txHash
  if (waitFor == WaitFor.BlockFinalization) {
    blockHash = result.status.asFinalized as H256
  } else {
    blockHash = result.status.asInBlock as H256
  }

  const header = await api.rpc.chain.getHeader(blockHash)
  const blockNumber: number = header.number.toNumber()

  return [txHash, txIndex, blockHash, blockNumber]
}

export class TransactionFailed {
  constructor(
    public reason: string,
    public details: TxResultDetails | null,
  ) {}
}

export interface TransactionOptions {
  app_id?: number
  nonce?: number
  era?: number
  blockHash?: H256
}
