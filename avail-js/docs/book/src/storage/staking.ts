import { SDK, Account } from "./../../../../src/index"

const main = async () => {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api

  // Staking Active Era
  const activeEra: any = await api.query.staking.activeEra()
  console.log(activeEra.__internal__raw.index.toNumber(0))
  console.log(activeEra.__internal__raw.start.toString())

  // Staking Bounded Iter
  const boundedIter = await sdk.api.query.staking.bonded.entries()
  for (const [key, value] of boundedIter) {
    console.log(key.toHuman())
    console.log(value.toString())
  }

  // Staking Bounded
  const bonded = await sdk.api.query.staking.bonded("5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY")
  console.log(bonded.toString())

  process.exit()
}
main()
