import { SDK, sdkUtil } from "./../../../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Converting hex string to H256 (BlockHash) using `hexStringToHash` interface.
  const hexString = "0x4a78c9fd1d88c99fc217eec0ac405307092e53523f6db19fae0242a5af9f4fe3"
  const maybeHex1 = sdk.util.hexStringToHash(hexString)
  if (maybeHex1.isErr()) {
    console.log("Error: " + maybeHex1.error)
    process.exit(1)
  }
  const hex1 = maybeHex1.value
  console.log(hex1.toHex()) // `0x4a78c9fd1d88c99fc217eec0ac405307092e53523f6db19fae0242a5af9f4fe3`

  // If we are sure that the hex string is valid.
  const hex2 = sdk.util.hexStringToHashUnsafe(hexString)
  console.log(hex2.toHex()) // `0x4a78c9fd1d88c99fc217eec0ac405307092e53523f6db19fae0242a5af9f4fe3`

  // Converting hex string to H256 (BlockHash) using `hexStringToHash` free function.
  const maybeHex3 = sdkUtil.hexStringToHash(sdk.api, hexString)
  if (maybeHex3.isErr()) {
    console.log("Error: " + maybeHex3.error)
    process.exit(1)
  }
  const hex3 = maybeHex3.value
  console.log(hex3.toHex()) // `0x4a78c9fd1d88c99fc217eec0ac405307092e53523f6db19fae0242a5af9f4fe3`

  // If we are sure that the hex string is valid.
  const hex4 = sdkUtil.hexStringToHashUnsafe(sdk.api, hexString)
  console.log(hex4.toHex()) // `0x4a78c9fd1d88c99fc217eec0ac405307092e53523f6db19fae0242a5af9f4fe3`

  // Converting from H256 to Hex String
  const hex = await sdk.api.rpc.chain.getFinalizedHead()
  const hexString2 = hex.toHex()
  console.log(hexString2) // `0xb410c0c0b5939567e5a558a4930ae030375894043c2dd5f3c35cea4133470f7f`

  // Error if the hex string doesn't start with 0x
  const hexString3 = "4c70c21c8b43d38cdf822d14733151ad2bcf48f9fdfe0d868852c11c9affbc81"
  const f1 = sdk.util.hexStringToHash(hexString3)
  if (f1.isErr()) {
    console.log("Error: " + f1.error) // `Failed to convert hex string to H256. Hash needs to start with 0x`
  }

  // Error if the hex string length is not 64 (without `0x`)
  const hexString4 = "0x4c70c21c8b43d38cdf822d1473315"
  const f2 = sdk.util.hexStringToHash(hexString4)
  if (f2.isErr()) {
    console.log("Error: " + f2.error) // `Error: Failed to convert hex string to H256. Expected length 64 got 29`
  }

  process.exit()
}
main()
