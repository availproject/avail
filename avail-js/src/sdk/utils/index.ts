import { ApiPromise } from "@polkadot/api"
import { err, ok, Result } from "neverthrow"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { decodeError } from "../../helpers"
import { FailedTxResult, getBlockHashAndTxHash, TxResultDetails, WaitFor } from "../transactions/common"
import { createKeyMulti, encodeAddress, sortAddresses } from "@polkadot/util-crypto"
import { H256 } from ".."
import { hexToU8a } from "@polkadot/util"
import { U8aFixed } from "@polkadot/types-codec"

export class Utils {
  private api: ApiPromise

  constructor(api: ApiPromise) {
    this.api = api
  }

  /// Parses a transaction result. Helper function to get transaction details on
  /// transaction success or an error if the transaction failed
  async parseTransactionResult(txResult: ISubmittableResult): Promise<Result<TxResultDetails, FailedTxResult>> {
    return await parseTransactionResult(this.api, txResult)
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

  /// Sorts multisig address so that ce be used for other multisig functions
  sortMultisigAddresses(addresses: string[]): string[] {
    return sortMultisigAddresses(addresses)
  }

  async getNonceState(address: string): Promise<number> {
    return await getNonceState(this.api, address)
  }

  async getNonceNode(address: string): Promise<number> {
    return await getNonceNode(this.api, address)
  }

  hexStringToHash(value: string): Result<H256, string> {
    return hexStringToHash(this.api, value)
  }
  hexStringToHashUnsafe(value: string): H256 {
    return hexStringToHashUnsafe(this.api, value)
  }
}

export async function parseTransactionResult(
  api: ApiPromise,
  txResult: ISubmittableResult,
): Promise<Result<TxResultDetails, FailedTxResult>> {
  if (txResult.isError) {
    if (txResult.status.isDropped) {
      return err(new FailedTxResult("Dropped", null))
    }

    if (txResult.status.isFinalityTimeout) {
      return err(new FailedTxResult("FinalityTimeout", null))
    }

    if (txResult.status.isInvalid) {
      return err(new FailedTxResult("Invalid", null))
    }

    if (txResult.status.isUsurped) {
      return err(new FailedTxResult("Usurped", null))
    }

    return err(new FailedTxResult("Error", null))
  }

  const events = txResult.events
  const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, api)
  const details = new TxResultDetails(txResult, events, txHash, txIndex, blockHash, blockNumber)

  const failed = txResult.events.find((e) => api.events.system.ExtrinsicFailed.is(e.event))
  if (failed != undefined) return err({ reason: decodeError(api, failed.event.data[0]), details })

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

export async function getNonceState(api: ApiPromise, address: string): Promise<number> {
  const r: any = await api.query.system.account(address)
  return parseInt(r.nonce.toString())
}

export async function getNonceNode(api: ApiPromise, address: string): Promise<number> {
  const r: any = await api.rpc.system.accountNextIndex(address)
  return parseInt(r.toString())
}

export function hexStringToHash(api: ApiPromise, value: string): Result<H256, string> {
  if (!value.startsWith("0x")) {
    return err("Failed to convert hex string to H256. Hash needs to start with 0x")
  }

  const hexString = value.slice(2)
  if (hexString.length != 64) {
    return err(`Failed to convert hex string to H256. Expected length 64 got ${hexString.length}.`)
  }

  const u8a = hexToU8a(hexString)
  const hex = new U8aFixed(api.registry, u8a)
  return ok(hex)
}

export async function getAppKeys(api: ApiPromise, address: string): Promise<number[]> {
  const appKeys: number[] = []
  const entries = await api.query.dataAvailability.appKeys.entries()
  entries.forEach((entry: any) => {
    if (entry[1].isSome) {
      const { owner, id } = entry[1].unwrap()
      if (owner.toString() == address) {
        appKeys.push(parseInt(id.toString()))
      }
    }
  })

  return appKeys.sort((a, b) => a - b)
}

export function hexStringToHashUnsafe(api: ApiPromise, value: string): H256 {
  const hash = hexStringToHash(api, value)
  if (hash.isErr()) {
    throw new Error(hash.error)
  }
  return hash.value
}

/**
 * Converts a hexadecimal string to an ASCII string.
 *
 * @param {string} hex - The hexadecimal string to convert.
 * @return {string} The converted ASCII string.
 */
export function fromHexToAscii(hex: string): string {
  let str = ""
  for (let n = 0; n < hex.length; n += 2) {
    str += String.fromCharCode(parseInt(hex.substring(n, n + 2), 16))
  }

  return `${str}`
}
