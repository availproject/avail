import { SDK } from "../../src/sdk"
import * as Balances from "./balances"
import * as DataAvailability from "./da"
import * as Staking from "./staking"
import * as Session from "./session"
import * as Multisig from "./multisig"
import * as NominationPools from "./nomination_pools"

async function main() {
  await Balances.run()
  await DataAvailability.run()
  await Staking.run()
  await Session.run()
  await Multisig.run()
  await NominationPools.run()
}

main()
  .catch((e) => {
    console.log(e)
  })
  .finally(() => {
    process.exit(0)
  })

export async function wait_for_new_era(value?: number) {
  console.log("Waiting for new era...")
  const sdk = await SDK.New(SDK.localEndpoint())
  let expectedEra = value || null

  while (true) {
    const activeEra: any = await sdk.api.query.staking.activeEra()
    let era = activeEra.__internal__raw.index.toNumber(0)

    if (expectedEra == null) {
      expectedEra = era + 1
    }

    if (era == expectedEra) {
      break
    }

    await delay(3000)
  }

  console.log("Waiting for new era... Done")
}

function delay(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}
