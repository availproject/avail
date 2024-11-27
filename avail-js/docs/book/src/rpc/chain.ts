import { SDK } from "./../../../../src/index"

const main = async () => {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api

  // Chain Get Block
  const block = await api.rpc.chain.getBlock()
  console.log(block.toJSON())

  // Chain Get Block Hash
  const hash = await api.rpc.chain.getBlockHash()
  console.log(hash.toJSON())

  // Chain Get Finalized Head
  const hash2 = await api.rpc.chain.getFinalizedHead()
  console.log(hash2.toJSON())

  // Chain Get Header
  const header = await api.rpc.chain.getHeader()
  console.log(header.toJSON())

  process.exit()
}
main()
