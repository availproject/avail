import { SDK, Keyring, Account } from "./../../../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Getting block hash for the latest (or specific) block
  const hash1 = await sdk.api.rpc.chain.getBlockHash()
  console.log(hash1.toHex()) // `0x4a78c9fd1d88c99fc217eec0ac405307092e53523f6db19fae0242a5af9f4fe3`

  // Getting block hash for the latest finalized block
  const hash2 = await sdk.api.rpc.chain.getFinalizedHead()
  console.log(hash2.toHex()) // `0x4a78c9fd1d88c99fc217eec0ac405307092e53523f6db19fae0242a5af9f4fe3`

  // Getting block hash from SignedBlock object
  const signedBlock = await sdk.api.rpc.chain.getBlock()
  const blockHashHexString = signedBlock.block.header.hash.toHex()
  const hash3 = sdk.util.hexStringToHashUnsafe(blockHashHexString)
  console.log(hash3.toHex()) // `0x4a78c9fd1d88c99fc217eec0ac405307092e53523f6db19fae0242a5af9f4fe3`

  // Getting block hash from sdk transaction
  const alice = new Account(sdk, new Keyring({ type: "sr25519" }).addFromUri("//Alice"))
  const tx = await alice.submitData("My Data") // this is equal to `sdk.tx.dataAvailability.submitData("My Data", WaitFor.BlockInclusion, aliceKeyring, options)`
  if (tx.isErr) {
    process.exit(1)
  }
  const hash4 = tx.blockHash
  console.log(hash4.toHex()) // `0x4a78c9fd1d88c99fc217eec0ac405307092e53523f6db19fae0242a5af9f4fe3`

  process.exit()
}
main()
