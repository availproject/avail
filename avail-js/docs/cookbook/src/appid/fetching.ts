import { SDK, Account, sdkUtil, Block, sdkBlock, WaitFor } from "./../../../../src/index"

const main = async () => {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api

  // Fetching the next app id via chain state query
  const nextAppIdCodec = await api.query.dataAvailability.nextAppId()
  const appId = parseInt(nextAppIdCodec.toString())
  console.log(appId) // 10

  // Fetching all app ids currently in use
  const appKeys = await api.query.dataAvailability.appKeys.entries()
  appKeys.forEach((entry: any) => {
    if (entry[1].isSome) {
      const { owner, id } = entry[1].unwrap()
      console.log(parseInt(id.toString())) // 1, 2, 0, 4, 3, ...
    }
  })

  // Fetching app id from transaction
  const alice = SDK.alice()
  const data = "My Data"
  const mtx = await sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, alice, { app_id: 1 })
  const tx = mtx._unsafeUnwrap()
  console.log(tx.appId) // 1

  // Fetching app id from Account instance transaction
  const account = new Account(sdk, alice)
  account.appId = 1
  const tx2 = (await account.submitData(data))._unsafeUnwrap()
  console.log(tx2.appId) // 1

  // Fetching all app ids owned by an account via account instance
  const appIds = await account.getAppKeys()
  console.log(appIds) // [0, 1, 2, 3,...]

  // Fetching all app ids owned by an account via free function
  const appIds2 = await sdkUtil.getAppKeys(api, alice.address)
  console.log(appIds2) // [0, 1, 2, 3,...]

  // Fetching app ids from a block via block instance
  const block = await Block.New(api, tx.details.blockHash)
  block.submitDataAll().forEach((ds) => console.log(ds.appId)) // 1

  // Fetching app ids from a block via free function
  const signedBlock = await api.rpc.chain.getBlock(tx.details.blockHash)
  sdkBlock.submitDataAll(signedBlock).forEach((ds) => console.log(ds.appId)) // 1

  // Manually from generic transaction
  block.transactionAll().forEach((tx) => console.log(sdkBlock.extractAppIdFromTx(tx))) // 0, 1, 0,...
  sdkBlock.transactionAll(signedBlock).forEach((tx) => console.log(sdkBlock.extractAppIdFromTx(tx))) // 0, 1, 0,...
  signedBlock.block.extrinsics.forEach((tx) => console.log(sdkBlock.extractAppIdFromTx(tx))) // 0, 1, 0,...

  process.exit()
}
main()
