import { initialize, getKeyringFromSeed, extractData } from "avail-js-sdk"
import config from "../../config"
import * as fs from 'fs';
import * as path from 'path';

/**
 * Example to submit data and retrieve the data from the block.
 */
const main = async () => {
  try {
    const data = fs.readFileSync("/root/code/avail/avail-js/examples/node-examples/src/2023-07-02 - 8mb string.txt", { encoding: 'utf8' });
    const api = await initialize(config.endpoint)
    const keyring = getKeyringFromSeed(config.seed)
    const options = { app_id: config.appId, nonce: -1 }

    await api.tx.dataAvailability.submitData(data).signAndSend(keyring, options)
    await api.tx.dataAvailability.submitData(data).signAndSend(keyring, options)
    await api.tx.dataAvailability.submitData(data).signAndSend(keyring, options)
    await api.tx.dataAvailability.submitData(data).signAndSend(keyring, options)
    // await api.tx.dataAvailability.submitData(data).signAndSend(keyring, options)
    // await api.tx.dataAvailability.submitData(data).signAndSend(keyring, options)
    // await api.tx.dataAvailability.submitData(data).signAndSend(keyring, options)
    // await api.tx.dataAvailability.submitData(data).signAndSend(keyring, options)
    console.log("sent")
    process.exit(0)
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()
