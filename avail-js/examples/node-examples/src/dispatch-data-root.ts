import { initialize, getKeyringFromSeed } from "avail-js-sdk"
import { ISubmittableResult } from "@polkadot/types/types"
import config from "../../config"

/**
 * Example using nomad, dispatching data root fot the particular block to the destination address and domain.
 */
const main = async () => {
  try {
    const api = await initialize(config.endpoint)
    const keyring = getKeyringFromSeed(config.seed)

    // submit one data transaction and wait until block is finalized
    // in order to dispatch data root once the block is finalized
    let res: ISubmittableResult = await new Promise(async (resolve) => {
      api.tx.dataAvailability.submitData("0x01").signAndSend(keyring, async (result) => {
        console.log(`Tx status: ${result.status}`)
        if (result.isFinalized) {
          console.log("Block is finalized.")
          resolve(result)
        }
      })
    })

    // destination domain always 1000
    const destinationDomain = 1000

    // data availability bridge router address deployed on Sepolia network e.g. 0x000000000000000000000000bD824890A51ed8bda53F51F27303b14EFfEbC152
    const bridgeRouterEthAddress = "0x000000000000000000000000bD824890A51ed8bda53F51F27303b14EFfEbC152"

    // hash of the block to dispatch data root
    const blockHash = res.status.asFinalized
    const header = await api.rpc.chain.getHeader(blockHash)

    // Send the transaction
    await api.tx.nomadDABridge
      .tryDispatchDataRoot(destinationDomain, bridgeRouterEthAddress, header)
      .signAndSend(keyring, (result) => {
        if (result.status.isInBlock) {
          console.log(`Tx hash: ${result.txHash} is in block ${result.status.asInBlock}`)
          process.exit(0)
        }
      })
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()
