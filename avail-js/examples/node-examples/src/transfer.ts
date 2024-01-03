import { getDecimals, initialize, formatNumberToBalance, getKeyringFromSeed, isValidAddress } from "avail-js-sdk"
import config from "../../config"

/**
 * Example of transfer with status end event tracking.
 */
const main = async () => {
  try {
    if (!isValidAddress(config.recipient)) throw new Error("Invalid Recipient")

    const api = await initialize(config.endpoint)
    const keyring = getKeyringFromSeed(config.seed)
    const options = { app_id: 0, nonce: -1 }
    const decimals = getDecimals(api)
    const amount = formatNumberToBalance(config.amount, decimals)

    await api.tx.balances.transfer(config.recipient, amount).signAndSend(keyring, options, ({ status, events }) => {
      if (status.isInBlock) {
        console.log(`Transaction included at blockHash ${status.asInBlock}`)
        events.forEach(({ event: { data, method, section } }) => {
          console.log(`\t' ${section}.${method}:: ${data}`)
        })
        process.exit(0)
      }
    })

    process.exit(0)
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()
