import { ApiPromise } from "@polkadot/api"
import { decodeAddress, encodeAddress, Keyring } from "@polkadot/keyring"
import { KeyringPair } from "@polkadot/keyring/types"
import { hexToU8a, isHex, BN, u8aToHex } from "@polkadot/util"

/**
 *
 * This function checks if a given address is valid.
 *
 * @param {string} address The address to validate.
 *
 * @returns {boolean} A boolean value indicating whether the address is valid or not.
 */
export const isValidAddress = (address: string): boolean => {
  try {
    encodeAddress(isHex(address) ? hexToU8a(address) : decodeAddress(address))
    return true
  } catch (error) {
    return false
  }
}

/**
 * Formats a number to balance.
 *
 * @param {number | string} value The number value to format.
 * @param {number} [decimals] The number of decimal places to include in the formatted balance. Defaults to 18.
 *
 * @returns {BN} The converted BN value.
 */
export const formatNumberToBalance = (value: number | string, decimals: number = 18): BN => {
  const MAX_NUMBER_VALUES = 10
  const [integerPart, fractionalPart] = value.toString().split(".")

  if (
    typeof value === "number" &&
    ((integerPart && integerPart.length > MAX_NUMBER_VALUES) ||
      (fractionalPart && fractionalPart.length > MAX_NUMBER_VALUES))
  ) {
    throw new Error("For big representation of number, please use a string instead of a number")
  }
  const integerBN = new BN(integerPart).mul(new BN(10).pow(new BN(decimals)))
  if (!fractionalPart) return integerBN
  const fractionalBN = new BN(`${fractionalPart}${"0".repeat(decimals)}`.slice(0, decimals))
  return integerBN.add(fractionalBN)
}

/**
 * Generates a new keyring.
 *
 * @returns {Keyring} The newly generated Keyring instance.
 */
export const generateKeyring = (): Keyring => {
  return new Keyring({ type: "sr25519" })
}

/**
 * Retrieves a keyring pair from a given seed.
 *
 * @param {string} seed The seed value used to generate the keypair.
 * @returns {KeyringPair} The KeyringPair generated from the seed.
 */
export const getKeyringFromSeed = (seed: string): KeyringPair => {
  const keyring = generateKeyring()
  return keyring.addFromUri(seed)
}

/**
 * Splits a string into an array of substrings of a specified chunk size.
 *
 * @param {string} inputString The input string to split.
 * @param {number} chunkSize The size of each chunk. Default is 2.
 * @returns {string[]} An array of substrings.
 */
export const splitStringIntoArray = (inputString: string, chunkSize: number = 2): string[] => {
  const result: string[] = []

  for (let i = 0; i < inputString.length; i += chunkSize) {
    result.push(inputString.substring(i, i + chunkSize))
  }

  return result
}

/**
 * Decodes a Uint8Array into a decimal value.
 *
 * @param {Uint8Array} value The Uint8Array to decode.
 * @returns {string} The decoded hex-encoded App ID as a string.
 */
export const decodeU8IntAppId = (value: Uint8Array): string => {
  const hexAppId = u8aToHex(value, undefined, false)
  return decodeHexAppId(hexAppId)
}

/**
 * Decodes a hex-encoded App ID string into a decimal value.
 *
 * @param {string} value The hex-encoded App ID string to decode.
 * @returns {string} The decoded decimal value as a string.
 * @throws {Error} If the input value has an invalid length.
 */
export const decodeHexAppId = (value: `0x${string}`): string => {
  if (value.length <= 1 || value.length % 2 !== 0) throw new Error("Invalid length")
  const v = value.startsWith("0x") ? value.substring(2) : value
  const array = splitStringIntoArray(v)
  let s = BigInt(0)
  array.forEach((x, i) => {
    s += BigInt(parseInt(x, 16)) << BigInt(i * 8)
  })
  const result = (s >> BigInt(array.length <= 4 ? 2 : 8)).toString()
  return result
}

/**
 * Extracts the data from a da submission
 *
 * @param {ApiPromise} api the api to interract with the chain.
 * @param {string} blockHash the hash of the block to query at.
 * @param {string} extrinsicHash the hash of the extrinsic to query at.
 * @return {Promise<string>} the bytes representing the data
 * @throws {Error} If the api is not connected, the block is empty or non existant, the extrinsic hash is non existant
 */
export const extractData = async (api: ApiPromise, blockHash: string, extrinsicHash: string): Promise<string> => {
  const block = await api.rpc.chain.getBlock(blockHash)
  const extrinsics = block.block.extrinsics.filter((x) => x.hash.toString() === extrinsicHash)
  if (extrinsics.length === 0) throw new Error("Extrinsic not found in block")
  const extrinsic = extrinsics[0]
  const {
    method: { args },
  } = extrinsic
  let dataHex = args.map((x) => x.toString()).join(", ")
  if (dataHex.startsWith("0x")) dataHex = dataHex.slice(2)
  let data = ""
  for (let n = 0; n < dataHex.length; n += 2) {
    data += String.fromCharCode(parseInt(dataHex.substring(n, n + 2), 16))
  }

  return data
}
