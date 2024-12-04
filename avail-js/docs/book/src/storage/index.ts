import { SDK } from "./../../../../src/index"

export async function run() {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api

  // dataAvailability.appKeys
  {
    const appKeyName = "Reserved-1"
    const entry = await api.query.dataAvailability.appKeys(appKeyName)
    if (!entry.isEmpty) {
      const appKey = JSON.parse(entry.toString())
      console.log(`App Key owner: ${appKey.owner}, id: ${appKey.id}`)
    }
  }

  // dataAvailability.appKeys.entries
  {
    const appKeys: [string, string, number][] = []
    const decoder = new TextDecoder("utf-8")
    const entries = await api.query.dataAvailability.appKeys.entries()
    entries.forEach((entry: any) => {
      if (entry[1].isSome) {
        const { owner, id } = entry[1].unwrap()
        appKeys.push([decoder.decode(entry[0].slice(49)), owner, parseInt(id.toString())])
      }
    })

    appKeys
      .sort((a, b) => a[2] - b[2])
      .forEach((e) => console.log(`App Key name: ${e[0]}, owner: ${e[1]}, id: ${e[2]}`))
  }

  // dataAvailability.nextAppId
  {
    const entry = await api.query.dataAvailability.nextAppId()
    if (!entry.isEmpty) {
      console.log(`Next App Id: ${parseInt(entry.toString())}`)
    }
  }

  // staking.activeEra
  {
    const entry: any = await api.query.staking.activeEra()
    console.log(entry.__internal__raw.index.toNumber(0))
    console.log(entry.__internal__raw.start.toString())
  }

  // staking.bonded
  {
    const entry = await api.query.staking.bonded("5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY")
    if (!entry.isEmpty) {
      console.log(`Bonded Stash: ${entry.toString()}`)
    }
  }

  // staking.bonded.entries
  {
    const entries = await api.query.staking.bonded.entries()
    for (const [key, value] of entries) {
      console.log(key.toHuman())
      console.log(value.toString())
    }
  }

  // system.account.entries
  {
    const entries = await api.query.system.account.entries()
    for (const [key, value] of entries) {
      const acc: any = value
      console.log(key.toHuman())
      console.log(acc.nonce.toNumber())
      console.log(acc.consumers.toNumber())
      console.log(acc.providers.toNumber())
      console.log(acc.sufficients.toNumber())
      console.log(acc.data.free.toString())
      console.log(acc.data.reserved.toString())
      console.log(acc.data.frozen.toString())
      console.log(acc.data.flags.toString())
    }
  }

  // system.account
  {
    const entry: any = await api.query.system.account("5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY")
    console.log(entry.nonce.toNumber())
    console.log(entry.consumers.toNumber())
    console.log(entry.providers.toNumber())
    console.log(entry.sufficients.toNumber())
    console.log(entry.data.free.toString())
    console.log(entry.data.reserved.toString())
    console.log(entry.data.frozen.toString())
    console.log(entry.data.flags.toString())
  }
}
