/// The example showcases how to programmatically become a nominator.
///
/// The following transactions are being called:
///   Utility.batchAll
///   Staking.bond
///   Session.nominate
///
/// The following storage are being queried:
///   Staking.bonded
///   Staking.ledger
///   Staking.nominators
///

import { initialize, getKeyringFromSeed } from "avail-js-sdk"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { H256 } from "@polkadot/types/interfaces/runtime"
import { BN } from "@polkadot/util"

import config from "../../config"

const main = async () => {
  try {
    const api = await initialize(config.endpoint)

    const account = getKeyringFromSeed(config.seed)
    const minNominatorBond = (await api.query.staking.minNominatorBond()).toString()
    // You can bond any amount of tokens as long as it is at least more or equal than the minimum
    // In this case we either bond the minimum amount or if there is no minimum we bond 1k AVAIL
    const bondAmount = new BN(minNominatorBond == "0" ? "1000000000000000000000" : minNominatorBond)
    // Here you can specify what targets will be nominated
    const targets = ["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"]

    const stakingBond = api.tx.staking.bond(bondAmount, "Staked")
    const stakingNominate = api.tx.staking.nominate(targets)

    // Transaction call
    const txResult = await new Promise<ISubmittableResult>((res) => {
      api.tx.utility.batchAll([stakingBond, stakingNominate]).signAndSend(account, (result: ISubmittableResult) => {
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
        // for module errors, we have the section indexed, lookup
        const decoded = api.registry.findMetaError(error.asModule)
        const { docs, name, section } = decoded
        console.log(`${section}.${name}: ${docs.join(" ")}`)
      } else {
        // Other, CannotLookup, BadOrigin, no extra info
        console.log(error.toString())
      }
      process.exit(1)
    }

    // Reading Nomination related information from storage
    const isBonded = (await api.query.staking.bonded(account.address)).toHuman()
    const ledger = await api.query.staking.ledger(account.address)
    const nominators = await api.query.staking.nominators(account.address)
    if (isBonded == undefined) {
      console.log("Something went wrong :(")
      process.exit(1)
    }

    console.log(`Staking.ledger: ${ledger}`)
    console.log(`Staking.nominators: ${nominators}`)

    process.exit(0)
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()
