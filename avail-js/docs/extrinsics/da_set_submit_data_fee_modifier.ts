import { SDK, WaitFor, Keyring, BN, sdkTransactions } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const modifier = {
    weightMaximumFee: new BN("10").pow(new BN("18")),
    weightFeeDivider: 20,
  } as sdkTransactions.DispatchFeeModifier

  const result = await sdk.tx.dataAvailability.setSubmitDataFeeModifier(modifier, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
