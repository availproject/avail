import { SDK, Events } from "./../../../../src/index"

export async function run() {
  const sdk = await SDK.New(SDK.localEndpoint())

  const account = SDK.alice()

  const dest = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
  const value = SDK.oneAvail()
  const tx = sdk.tx.balances.transferKeepAlive(dest, value)
  const res = (await tx.executeWaitForInclusion(account))._unsafeUnwrap()

  for (const eventRecord of res.events) {
    console.log(`Pallet name: ${eventRecord.event.section}, Event name: ${eventRecord.event.method}`)
  }

  // findFirstEvent, findLastEvent, findEvent
  const event = res.findFirstEvent(Events.Balances.Transfer)
  if (event == null) throw Error("qed")

  console.log(`Transfer from: ${event.from}, to: ${event.to}, amount: ${event.amount}`)
}
