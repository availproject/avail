/* eslint-disable  @typescript-eslint/no-explicit-any */
/// The example showcases how to programmatically create application key.
///
/// The following transactions are being called:
///   DataAvailability.createApplicationKey
///

import { initialize, getKeyringFromSeed } from "avail-js-sdk"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { H256 } from "@polkadot/types/interfaces/runtime"

import config from "../../config"

const main = async () => {
  try {
    const api = await initialize(config.endpoint)
    const account = getKeyringFromSeed(config.seed)

    // Transaction call
    const key = "My Key"
    const txResult = await new Promise<ISubmittableResult>((res) => {
      api.tx.dataAvailability.createApplicationKey(key).signAndSend(account, (result: ISubmittableResult) => {
        console.log(`Tx status: ${result.status}`)
        if (result.isFinalized || result.isError) {
          res(result)
        }
      })
    })

    // Rejected Transaction handling
    if (txResult.isError) {
      console.log(`Transaction was not executed`)
      process.exit(1)
    }

    const [txHash, blockHash] = [txResult.txHash as H256, txResult.status.asFinalized as H256]
    console.log(`Tx Hash: ${txHash}, Block Hash: ${blockHash}`)

    // Failed Transaction handling
    const error = txResult.dispatchError
    if (error != undefined) {
      if (error.isModule) {
        const decoded = api.registry.findMetaError(error.asModule)
        const { docs, name, section } = decoded
        console.log(`${section}.${name}: ${docs.join(" ")}`)
      } else {
        console.log(error.toString())
      }
      process.exit(1)
    }

    const e = txResult.events.find((e) => e.event.method == "ApplicationKeyCreated")
    if (e == undefined) {
      console.log(`Missing ApplicationKeyCreated method.`)
      process.exit(1)
    }

    const data: any = e.event.data
    const readKey = data["key"].toString()
    const readOwner = data["owner"].toString()
    const readAppId = data["id"].toString()
    console.log(`
      key=${readKey},
      owner=${readOwner},
      id=${readAppId},
    `)

    process.exit(0)
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()
