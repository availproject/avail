import { initialize, getKeyringFromSeed } from "avail-js-sdk"
import config from "../../config"

/**
 * Example of creating a technical committee proposal.
 */
const main = async () => {
  try {
    const api = await initialize(config.endpoint)
    const keyring = getKeyringFromSeed(config.seed)

    // initiate the transaction to, for example, increase the validator count
    const increaseValidatorCount = api.tx.staking.increaseValidatorCount(10)

    // create the proposal from the transaction
    const proposal = api.registry.createType("Call", {
      callIndex: increaseValidatorCount.callIndex,
      args: increaseValidatorCount.args,
    })

    // create the motion from the proposal
    const motion = {
      proposal: proposal,
      threshold: api.registry.createType("Compact<u32>", 1),
      length_bound: api.registry.createType("Compact<u32>", 36),
    }

    // create the council proposal transaction
    const tx = api.tx.technicalCommittee.propose(motion.threshold, motion.proposal, motion.length_bound)

    // submit the transaction
    const unsub = await tx.signAndSend(keyring, ({ status }) => {
      console.log("Transaction status:", status.type)

      if (status.isInBlock) {
        console.log("Completed at block hash", status.asInBlock.toHex())
        unsub()
        process.exit(0)
      }
    })
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()
