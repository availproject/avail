import { SDK, Account, BN } from "./../../../../src/index"

const main = async () => {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api
  const alice = Account.alice(sdk)

  // System Account Next Index
  const nodeNonce: BN = await api.rpc.system.accountNextIndex(alice.address())
  console.log(nodeNonce.toNumber())

  // System Chain
  const chain = await api.rpc.system.chain()
  console.log(chain.toString())

  // System Chain Type
  const chainType = await api.rpc.system.chainType()
  console.log(chainType.toString())

  // System Health
  const health = await api.rpc.system.health()
  console.log(health.peers.toNumber())
  console.log(health.isSyncing.toString())
  console.log(health.shouldHavePeers.toString())

  // System Local Listen Addresses
  const localListenAddresses = await api.rpc.system.localListenAddresses()
  localListenAddresses.forEach((e) => console.log(e.toString()))

  // System Local Peer Id
  const localPeerId = await api.rpc.system.localPeerId()
  console.log(localPeerId.toString())

  // System Name
  const name = await api.rpc.system.name()
  console.log(name.toString())

  // System Node Roles
  const nodeRoles = await api.rpc.system.nodeRoles()
  nodeRoles.forEach((e) => console.log(e.toString()))

  // System Peers
  const peers = await api.rpc.system.peers()
  peers.forEach((e) => console.log(e.toString()))

  // System Properties
  const properties = await api.rpc.system.properties()
  console.log(properties.isEthereum.toString())
  console.log(properties.ss58Format.toString())
  if (properties.tokenDecimals.isSome) {
    properties.tokenDecimals.value.forEach((e) => console.log(e.toString()))
  }
  if (properties.tokenSymbol.isSome) {
    properties.tokenSymbol.value.forEach((e) => console.log(e.toString()))
  }

  // System Sync State
  const syncState = await api.rpc.system.syncState()
  console.log(syncState.startingBlock.toNumber())
  console.log(syncState.currentBlock.toNumber())
  if (syncState.highestBlock.isSome) {
    console.log(syncState.highestBlock.value.toNumber())
  }

  // System Version
  const version = await api.rpc.system.version()
  console.log(version.toString())

  process.exit()
}
main()
