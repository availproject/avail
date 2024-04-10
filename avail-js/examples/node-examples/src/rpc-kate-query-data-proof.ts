/* eslint-disable  @typescript-eslint/no-explicit-any */
/// The example showcases how to programmatically call query data proof RPC.
///

import { initialize, getKeyringFromSeed } from "avail-js-sdk"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { H256 } from "@polkadot/types/interfaces/runtime"

import config from "../../config"

const main = async () => {
  try {
    const api = await initialize(config.endpoint)
    const account = getKeyringFromSeed(config.seed)
    const appId = config.appId === 0 ? 1 : config.appId
    const options = { app_id: appId, nonce: -1 }

    // submit data transaction
    const txResult = await new Promise<ISubmittableResult>((res) => {
      api.tx.dataAvailability.submitData("Hello World").signAndSend(account, options, (result: ISubmittableResult) => {
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

    const rpc: any = api.rpc
    const dataProof = await rpc.kate.queryDataProof(1, blockHash)
    console.log(`Header: ${JSON.stringify(dataProof, undefined, 2)}`)

    process.exit(0)
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()
