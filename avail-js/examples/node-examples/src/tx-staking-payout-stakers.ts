import { ISubmittableResult } from "@polkadot/types/types"
import { H256 } from "@polkadot/types/interfaces/runtime"
import { getKeyringFromSeed, initialize, disconnect } from "avail-js-sdk"
import config from "../../config"

/**
 * Script to automate payouts.
 */
const main = async () => {
  const api = await initialize(config.endpoint)

  /* THINGS YOU CAN CHANGE */
  // The account you'll do the payout with
  const account = getKeyringFromSeed(config.seed)
  const options = { app_id: 0, nonce: -1 }

  // Put the list of validators you want to do the payout for, leave empty for all of them
  const validatorStashes: string[] = []

  // Number of days to check (how many era we'll check for pending rewards)
  let nbEra = 1

  /* THINGS YOU SHOULD NOT CHANGE */
  // Get the active era
  const activeEra = (await api.query.staking.currentEra()).toJSON() as number

  // Set the starting era to scrap reward
  if (nbEra > 84) nbEra = 84
  const startEra = activeEra > nbEra ? activeEra - nbEra : 0

  // We set a list of eras and validators to claim for
  let toClaim: { era: number; validator: string }[] = []

  for (let i = startEra; i < activeEra; i++) {
    // We get the validators who earned reward during this era
    const eraRewardPoints = (await api.query.staking.erasRewardPoints(i)).toJSON() as {
      total: number
      individual: { [address: string]: number }
    }
    const eraRewardPointsValidatorList = Object.keys(eraRewardPoints.individual)

    // We get the validators where the payout has already been done for this era
    const claimedRewards = (await api.query.staking.claimedRewards.entries(i)).map(
      (x) => (x[0].toHuman() as string[])[1],
    )

    // We get all validator WITH eraRewardPoints and WITHOUT already claimed reward
    let validatorsWithPendingClaim = eraRewardPointsValidatorList.filter((x) => !claimedRewards.includes(x))

    // We filter by the specified stashes if there are any
    if (validatorStashes.length > 0) {
      validatorsWithPendingClaim = validatorsWithPendingClaim.filter((x) => validatorStashes.includes(x))
    }

    // We update the global list
    toClaim = [
      ...toClaim,
      ...validatorsWithPendingClaim.map((x) => {
        return { era: i, validator: x }
      }),
    ]
    console.log(`Found ${validatorsWithPendingClaim.length} validators with pending claims for era ${i}`)
  }

  // We create all the transactions
  const transactions = await Promise.all(toClaim.map((x) => api.tx.staking.payoutStakers(x.validator, x.era)))
  const chunks = []
  const chunkSize = 5
  for (let i = 0; i < transactions.length; i += chunkSize) {
    const chunk = transactions.slice(i, i + chunkSize)
    chunks.push(chunk)
  }

  // We batch them together
  const batches = chunks.map((x) => api.tx.utility.batchAll(x))
  for (const [i, tx] of batches.entries()) {
    console.log(`Sending batch transaction ${i + 1} of ${batches.length}`)

    // Send the batch
    const txResult = await new Promise<ISubmittableResult>((res) => {
      tx.signAndSend(account, options, (result: ISubmittableResult) => {
        if (result.isInBlock || result.isError) {
          res(result)
        }
      })
    })

    // Error handling
    if (!txResult.isError) {
      console.log(`Payout done successfully for batch transaction ${i + 1} of ${batches.length} `)
      console.log(`Tx Hash: ${txResult.txHash as H256}, Block Hash: ${txResult.status.asInBlock as H256}`)
    } else {
      console.log(`Transaction was not executed for batch transaction ${i + 1} of ${batches.length}`)
    }
  }

  console.log("Everything was done, bye !")
  await disconnect()
  process.exit(0)
}
main()
