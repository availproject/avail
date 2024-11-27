import { BN, SDK, Transaction, Events } from "./../../../../src/index"

export async function run() {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api

  const account = SDK.alice()

  const value1 = SDK.oneAvail()
  const value2 = SDK.oneAvail().mul(new BN("100000000"))
  const destBob = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
  const destCharlie = "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"

  const call1 = api.tx.balances.transferKeepAlive(destBob, value1)
  const call2 = api.tx.balances.transferKeepAlive(destCharlie, value2)
  const calls = [call1, call2]

  // Batch
  // This will return `Ok` in all circumstances. To determine the success of the batch, an
  // event is deposited. If a call failed and the batch was interrupted, then the
  // `BatchInterrupted` event is deposited, along with the number of successful calls made
  // and the error of the failed call. If all were successful, then the `BatchCompleted`
  // event is deposited.
  const batchTx = new Transaction(api, api.tx.utility.batch(calls))
  const batchRes = (await batchTx.execute_wait_for_inclusion(account))._unsafeUnwrap()
  console.log("-- Batch Call --")

  const batchInterrupted = batchRes.findAllEvents(Events.Utility.BatchInterrupted)
  if (batchInterrupted.length > 0) {
    console.log("At least one call has failed")
  }

  const batchCompleted1 = batchRes.findFirstEvent(Events.Utility.BatchCompleted)
  if (batchCompleted1 != null) {
    console.log("All calls were successful")
  }

  // Batch All
  // Send a batch of dispatch calls and atomically execute them.
  // The whole transaction will rollback and fail if any of the calls failed.
  const batchAllTx = new Transaction(api, api.tx.utility.batchAll(calls))
  const _ = (await batchAllTx.execute_wait_for_inclusion(account))._unsafeUnwrapErr()

  // Force Batch
  // Send a batch of dispatch calls.
  // Unlike `batch`, it allows errors and won't interrupt.
  const forceBatchTx = new Transaction(api, api.tx.utility.forceBatch(calls))
  const forceBatchRes = (await forceBatchTx.execute_wait_for_inclusion(account))._unsafeUnwrap()
  console.log("-- Force Batch Call --")

  const itemFailed = forceBatchRes.findAllEvents(Events.Utility.ItemFailed)
  if (itemFailed.length > 0) {
    console.log("At least one call has failed")
  }

  const batchCompletedWithErrors = forceBatchRes.findFirstEvent(Events.Utility.BatchCompletedWithErrors)
  if (batchCompletedWithErrors != null) {
    console.log("Batch completed even though one or more calls have failed")
  }

  const batchCompleted2 = forceBatchRes.findFirstEvent(Events.Utility.BatchCompleted)
  if (batchCompleted2 != null) {
    console.log("All calls were successful")
  }
}
