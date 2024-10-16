import { ApiPromise } from "@polkadot/api"
import { err, ok, Result } from "neverthrow"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { EventRecord, H256, Weight } from "@polkadot/types/interfaces"
import { decodeError } from "../../helpers"
import { getBlockHashAndTxHash, standardCallback, WaitFor } from "../transactions/common"
import { createKeyMulti, encodeAddress, sortAddresses } from "@polkadot/util-crypto"
import { KeyringPair } from "@polkadot/keyring/types"
import { SignerOptions } from "@polkadot/api/types"

export class TxResultDetails {
  constructor(
    public txResult: ISubmittableResult,
    public events: EventRecord[],
    public txHash: H256,
    public txIndex: number,
    public blockHash: H256,
    public blockNumber: number,
  ) {}
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

export class Utils {
  private api: ApiPromise

  constructor(api: ApiPromise) {
    this.api = api
  }

  /// Parses a transaction result. Helper function to get transaction details on
  /// transaction success or an error if the transaction failed
  async parseTransactionResult(
    txResult: ISubmittableResult,
    waitFor: WaitFor,
  ): Promise<Result<TxResultDetails, FailedTxResult>> {
    return await parseTransactionResult(this.api, txResult, waitFor)
  }

  /**
   * Converts a commission percentage to a perbill format.
   *
   * @param {number} value - The commission percentage (0-100).
   * @return {string} The commission value in perbill format.
   * @throws {Error} If the value is not an integer or is out of the 0-100 range.
   */
  commissionNumberToPerbill(value: number): Result<string, string> {
    return commissionNumberToPerbill(value)
  }

  /// Generates a multisig account
  generateMultisig(addresses: string[], threshold: number): string {
    return generateMultisig(addresses, threshold)
  }

  /// Creates and approves a multisig transaction
  async firstMultisigApproval(
    callHash: string,
    threshold: number,
    otherSignatures: string[],
    maxWeight: Weight,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: Partial<SignerOptions>,
  ): Promise<Result<TxResultDetails, string>> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.multisig
        .approveAsMulti(threshold, otherSignatures, null, callHash, maxWeight)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return err(maybeTxResult.error)
    }
    const txResult = maybeTxResult.value
    const maybeParsed = await this.parseTransactionResult(txResult, waitFor)
    if (maybeParsed.isErr()) {
      return err(maybeParsed.error.reason)
    }
    const parsed = maybeParsed.value

    return ok(parsed)
  }

  /// Approves an existing multisig transaction
  async nextMultisigApproval(
    callHash: string,
    threshold: number,
    otherSignatures: string[],
    timepoint: MultisigTimepoint,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: Partial<SignerOptions>,
  ): Promise<Result<TxResultDetails, string>> {
    const maxWeight = { refTime: 0, proofSize: 0 }
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.multisig
        .approveAsMulti(threshold, otherSignatures, timepoint, callHash, maxWeight)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return err(maybeTxResult.error)
    }
    const txResult = maybeTxResult.value
    const maybeParsed = await this.parseTransactionResult(txResult, waitFor)
    if (maybeParsed.isErr()) {
      return err(maybeParsed.error.reason)
    }
    const parsed = maybeParsed.value

    return ok(parsed)
  }

  /// Approves and executes an existing multisig transaction
  async lastMultisigApproval(
    threshold: number,
    otherSignatures: string[],
    timepoint: MultisigTimepoint,
    callData: string,
    maxWeight: Weight,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: Partial<SignerOptions>,
  ): Promise<Result<TxResultDetails, string>> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.multisig
        .asMulti(threshold, otherSignatures, timepoint, callData, maxWeight)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return err(maybeTxResult.error)
    }
    const txResult = maybeTxResult.value
    const maybeParsed = await this.parseTransactionResult(txResult, waitFor)
    if (maybeParsed.isErr()) {
      return err(maybeParsed.error.reason)
    }
    const parsed = maybeParsed.value

    return ok(parsed)
  }

  /// Sorts multisig address so that ce be used for other multisig functions
  sortMultisigAddresses(addresses: string[]): string[] {
    return sortMultisigAddresses(addresses)
  }
}

export async function parseTransactionResult(
  api: ApiPromise,
  txResult: ISubmittableResult,
  waitFor: WaitFor,
): Promise<Result<TxResultDetails, FailedTxResult>> {
  if (txResult.isError) {
    return err({ reason: "The transaction was dropped or something.", details: null })
  }

  const events = txResult.events
  const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, api)
  const details = new TxResultDetails(txResult, events, txHash, txIndex, blockHash, blockNumber)

  const failed = txResult.events.find((e) => api.events.system.ExtrinsicFailed.is(e.event))
  if (failed != undefined) {
    return err({ reason: decodeError(api, failed.event.data[0]), details })
  }

  return ok(details)
}

export function commissionNumberToPerbill(value: number): Result<string, string> {
  if (!Number.isInteger(value)) {
    return err("Commission cannot have decimal place. It needs to be a whole number.")
  }

  if (value < 0 || value > 100) {
    return err("Commission is limited to the following range: 0 - 100. It cannot be less than 0 or more than 100.")
  }

  let commission = value.toString().concat("0000000")
  // For some reason 0 commission is not defined as "0" but as "1".
  if (commission == "00000000") {
    commission = "1"
  }

  return ok(commission)
}

export function generateMultisig(addresses: string[], threshold: number): string {
  const SS58Prefix = 42

  const multiAddress = createKeyMulti(addresses, threshold)
  const Ss58Address = encodeAddress(multiAddress, SS58Prefix)

  return Ss58Address
}

export function sortMultisigAddresses(addresses: string[]): string[] {
  const SS58Prefix = 42

  return sortAddresses(addresses, SS58Prefix)
}
