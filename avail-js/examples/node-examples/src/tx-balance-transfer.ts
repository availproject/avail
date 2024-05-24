/* eslint-disable  @typescript-eslint/no-explicit-any */
/// The example showcases how to programmatically do balance transfer.
///
/// The following transactions are being called:
///   Balance.transfer
///
/// The following storage are being queried:
///   System.account
///

import { getDecimals, initialize, formatNumberToBalance, getKeyringFromSeed, isValidAddress } from "avail-js-sdk"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { H256 } from "@polkadot/types/interfaces/runtime"

import config from "../../config"

const main = async () => {
  try {
    if (!isValidAddress(config.recipient)) throw new Error("Invalid Recipient")

    const api = await initialize(config.endpoint)
    const keyring = getKeyringFromSeed(config.seed)
    const options = { app_id: 0, nonce: -1 }
    const decimals = getDecimals(api)
    const amount = formatNumberToBalance(config.amount, decimals)

    const oldBalance: any = await api.query.system.account(config.recipient)
    console.log(`Balance before the transfer call: ${oldBalance["data"]["free"].toHuman()}`)

    // Transaction call
    const txResult = await new Promise<ISubmittableResult>((res) => {
      api.tx.balances
        .transferKeepAlive(config.recipient, amount)
        .signAndSend(keyring, options, (result: ISubmittableResult) => {
          console.log(`Tx status: ${result.status}`)
          if (result.isFinalized || result.isError) {
            res(result)
          }
        })
    })
    console.log(`Tx Hash: ${txResult.txHash as H256}, Block Hash: ${txResult.status.asFinalized as H256}`)

    // Error handling
    const error = txResult.dispatchError
    if (txResult.isError) {
      console.log(`Transaction was not executed`)
    } else if (error != undefined) {
      if (error.isModule) {
        const decoded = api.registry.findMetaError(error.asModule)
        const { docs, name, section } = decoded
        console.log(`${section}.${name}: ${docs.join(" ")}`)
      } else {
        console.log(error.toString())
      }
      process.exit(1)
    }

    const newBalance: any = await api.query.system.account(config.recipient)
    console.log(`Balance after the transfer call: ${newBalance["data"]["free"].toHuman()}`)

    process.exit(0)
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()
