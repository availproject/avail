/// The example showcases how to programmatically become a validator.
///
/// The following transactions are being called:
///   Utility.batchAll
///   Staking.bond
///   Session.set_key
///   Staking.validate
///
/// The following storage are being queried:
///   Staking.bonded
///   Staking.ledger
///   Staking.validators
///

import { initialize, getKeyringFromSeed } from "avail-js-sdk"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { H256 } from "@polkadot/types/interfaces/runtime"
import { BN } from "@polkadot/util"

import config from "../../config"

function deconstruct_session_keys(deconstruct_session_keys: string) {
  const keys = deconstruct_session_keys.slice(2, undefined)
  const babeKey = "0x".concat(keys.slice(0, 64))
  const grandpaKey = "0x".concat(keys.slice(64, 128))
  const imonlineKey = "0x".concat(keys.slice(128, 192))
  const authorityDiscoveryKey = "0x".concat(keys.slice(192, 256))

  return {
    babe: babeKey,
    grandpa: grandpaKey,
    imOnline: imonlineKey,
    authorityDiscover: authorityDiscoveryKey,
  }
}

function define_validator_preference() {
  // "5" means 5 percent.
  let commission = "5".concat("0000000")
  // For some reason 0 commission is not defined as "0" but as "1".
  if (commission == "00000000") {
    commission = "1"
  }
  return { commission: commission, block: false }
}

const main = async () => {
  try {
    const api = await initialize(config.endpoint)

    const account = getKeyringFromSeed(config.seed)
    const minValidatorBond = (await api.query.staking.minValidatorBond()).toString()
    // You can bond any amount of tokens as long as it is at least more or equal than the minimum
    // In this case we either bond the minimum amount or if there is no minimum we bond 1k AVAIL
    const bondAmount = new BN(minValidatorBond == "0" ? "1000000000000000000000" : minValidatorBond)
    // You need to generate the session keys yourself and put the value in here
    const sessionKeys = config.sessionKeys
    const keys = deconstruct_session_keys(sessionKeys)
    const prefs = define_validator_preference()

    const stakingBond = api.tx.staking.bond(bondAmount, "Staked")
    const sessionSetKeys = api.tx.session.setKeys(keys, undefined)
    const stakingValidate = api.tx.staking.validate(prefs)

    // Transaction call
    const txResult = await new Promise<ISubmittableResult>((res) => {
      api.tx.utility
        .batchAll([stakingBond, sessionSetKeys, stakingValidate])
        .signAndSend(account, (result: ISubmittableResult) => {
          console.log(`Tx status: ${result.status}`)
          if (result.isFinalized || result.isError) {
            res(result)
          }
        })
    })

    // Rejected Transaction handling
    if (txResult.isError) {
      console.log(`Transaction was not executed`)
      process.exit(0)
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

    // Reading Validator related information from storage
    const isBonded = (await api.query.staking.bonded(account.address)).toHuman()
    const ledger = await api.query.staking.ledger(account.address)
    const validators = await api.query.staking.validators(account.address)
    if (isBonded == undefined) {
      console.log("Something went wrong :(")
      process.exit(1)
    }

    console.log(`Staking.ledger: ${ledger}`)
    console.log(`Staking.validators: ${validators}`)

    process.exit(0)
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()
