import { disconnect, initialize, isConnected, getDecimals } from "."

test("It connects to the default network", async () => {
  const api = await initialize()
  const chain = await api.rpc.system.chain()
  expect(chain).toBeDefined()
  expect(isConnected()).toBe(true)
})

test("It fetches the latest block", async () => {
  const api = await initialize()
  const block = await api.rpc.chain.getBlock()
  console.log("Latest block number: ", block.block.header.number.toNumber())
  expect(block).toBeDefined()
  await disconnect()
})

test("It gets the correct number of decimals", async () => {
  const api = await initialize()
  const decimals = getDecimals(api)
  expect(decimals).toEqual(18)
})
