import { SDK, BN, utils } from "./../../../../src/index"

export async function run() {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api

  const account = SDK.alice()

  // Bond minValidatorBond or 1 AVAIL token
  const minValidatorBond: BN = ((await api.query.staking.minValidatorBond()) as any) || SDK.oneAvail()

  // Bond
  const bondTx = sdk.tx.staking.bond(minValidatorBond, "Staked")
  const _r1 = (await bondTx.executeWaitForInclusion(account))._unsafeUnwrap()

  // Generate Session Keys
  const keysBytes = await api.rpc.author.rotateKeys()
  const keys = utils.deconstruct_session_keys(keysBytes.toString())

  // Set Keys
  const setKeysTx = sdk.tx.session.setKeys(keys)
  const _r2 = (await setKeysTx.executeWaitForInclusion(account))._unsafeUnwrap()

  // Validate
  const validateTx = sdk.tx.staking.validate(50, false)
  const _r3 = (await validateTx.executeWaitForInclusion(account))._unsafeUnwrap()
}
