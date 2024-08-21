import { Keyring } from "@polkadot/api"
import { BN } from "@polkadot/util"
import { SDK } from "avail-js-sdk"
import { WaitFor } from "avail-js-sdk/sdk/transactions"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice//stash")
  const maxAdditional = new BN(10).pow(new BN(18)) // one Avail

  const result = await sdk.tx.staking.bondExtra(maxAdditional, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log("Stash=" + result.event.stash + ", Amount=" + result.event.amount)
  console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash)

  process.exit()
}
main()
