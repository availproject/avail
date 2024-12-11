import { SDK, utils } from "./../../../../src/index"

export async function run() {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api

  // Converting from H256 to Hex String
  const hash = await api.rpc.chain.getFinalizedHead()
  console.log(hash.toHex()) // `0xb410c0c0b5939567e5a558a4930ae030375894043c2dd5f3c35cea4133470f7f`

  const hexString = "0x4a78c9fd1d88c99fc217eec0ac405307092e53523f6db19fae0242a5af9f4fe3"

  // Converting hex string to H256 (BlockHash) via free function safe
  const hex3 = utils.hexStringToHash(api, hexString)
  if (hex3.isErr()) throw Error(hex3.error)
  console.log(hex3.value.toHex()) // `0x4a78c9fd1d88c99fc217eec0ac405307092e53523f6db19fae0242a5af9f4fe3`

  // Converting hex string to H256 (BlockHash) via free function unsafe
  const hex4 = utils.hexStringToHashUnsafe(api, hexString)
  console.log(hex4.toHex()) // `0x4a78c9fd1d88c99fc217eec0ac405307092e53523f6db19fae0242a5af9f4fe3`

  // Error if the hex string doesn't start with 0x
  const hexString2 = "4c70c21c8b43d38cdf822d14733151ad2bcf48f9fdfe0d868852c11c9affbc81"
  const f1 = utils.hexStringToHash(api, hexString2)
  if (f1.isOk()) throw Error("Expected to fail")
  console.log("Error: " + f1.error) // `Failed to convert hex string to H256. Hash needs to start with 0x`

  // Error if the hex string length is not 64 (without `0x`)
  const hexString3 = "0x4c70c21c8b43d38cdf822d1473315"
  const f2 = utils.hexStringToHash(api, hexString3)
  if (f2.isOk()) throw Error("Expected to Fail")
  console.log("Error: " + f2.error) // `Error: Failed to convert hex string to H256. Expected length 64 got 29`
}
