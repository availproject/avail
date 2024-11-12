import { SDK, Keyring, Account, WaitFor } from "./../../../../src/index"

const main = async () => {
  const sdk = await SDK.New(SDK.localEndpoint())
  const alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice")

  /*
      Sometimes the existing sdk functions can be too low level and cumbersome to use.
      The Account class tries to remedy that by providing a similar interface but
      with less parameters. 
  
      Account related data like balance, nonce and associated app keys can all be accessed
      by calling `getBalance`, `getNonceState`, and `getAppKeys` respectively. 
    */
  const account = new Account(sdk, alice)
  console.log((await account.getBalance()).free.toString())
  console.log(await account.getNonceNode())
  console.log(await account.getNonceState())
  console.log(await account.getAppKeys())
  console.log(await account.getAppIds())

  /*
      Three most common transactions are as well part of the Account interface.
      `balanceTransfer` (`balanceTransferNoWait`)
      `createApplicationKey` (`createApplicationKeyNoWait`)
      `submitData` (`submitDataNoWait`)
    */
  const _r1 = await account.balanceTransfer("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", account.oneAvail())
  const r2 = (await account.createApplicationKey("Alice Key"))._unsafeUnwrap()

  /*
      Setting app id, nonce, and the waitfor method can be easily done by calling
      `setAppId`, `setNonce`, and `setWaitFor` respectively.
  
      These values are sticky which means they will persist until you change them again.
    */
  account.setAppId(r2.event.id)
  account.setNonce(await account.getNonceNode())
  account.setWaitFor(WaitFor.BlockInclusion)
  const _r3 = await account.submitData("My Data")

  /*
      To make sure that we don't use the same app id and the same nonce for our next
      call we reset them to null (default value)
    */
  account.setAppId(null)
  account.setNonce(null)
  const _r4 = await account.submitDataNoWait("My Data")

  process.exit()
}
main()
