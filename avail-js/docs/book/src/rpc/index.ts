import { BN, InclusionFee, SDK, utils } from "./../../../../src/index"

export async function run() {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api

  // ANCHOR: author_rotateKeys
  // author.rotateKeys
  const keysBytes = await api.rpc.author.rotateKeys()
  const keys = utils.deconstruct_session_keys(keysBytes.toString())
  console.log(keys)
  // ANCHOR_END: author_rotateKeys

  // ANCHOR: chain_getBlock
  // chain.getBlock
  const block = await api.rpc.chain.getBlock()
  console.log(block.toJSON())
  // ANCHOR_END: chain_getBlock

  // ANCHOR: chain_getBlockHash
  // chain.getBlockHash
  const hash = await api.rpc.chain.getBlockHash()
  console.log(hash.toJSON())
  // ANCHOR_END: chain_getBlockHash

  // ANCHOR: chain_getFinalizedHead
  // chain.getFinalizedHead
  const hash2 = await api.rpc.chain.getFinalizedHead()
  console.log(hash2.toJSON())
  // ANCHOR_END: chain_getFinalizedHead

  // ANCHOR: chain_getHeader
  // chain.getHeader
  const header = await api.rpc.chain.getHeader()
  console.log(header.toJSON())
  // ANCHOR_END: chain_getHeader

  // ANCHOR: system_accountNextIndex
  // system.accountNextIndex
  const address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
  const nodeNonce: BN = await api.rpc.system.accountNextIndex(address)
  console.log(nodeNonce.toNumber())
  // ANCHOR_END: system_accountNextIndex

  // ANCHOR: system_chain
  // system.chain
  const chain = await api.rpc.system.chain()
  console.log(chain.toJSON())
  // ANCHOR_END: system_chain

  // ANCHOR: system_chainType
  // system.chainType
  const chainType = await api.rpc.system.chainType()
  console.log(chainType.toString())
  // ANCHOR_END: system_chainType

  // ANCHOR: system_health
  // system.health
  const health = await api.rpc.system.health()
  console.log(health.peers.toNumber())
  console.log(health.isSyncing.toString())
  console.log(health.shouldHavePeers.toString())
  // ANCHOR_END: system_health

  // ANCHOR: system_localListenAddresses
  // system.localListenAddresses
  const localListenAddresses = await api.rpc.system.localListenAddresses()
  localListenAddresses.forEach((e) => console.log(e.toString()))
  // ANCHOR_END: system_localListenAddresses

  // ANCHOR: system_localPeerId
  // system.localPeerId
  const localPeerId = await api.rpc.system.localPeerId()
  console.log(localPeerId.toJSON())
  // ANCHOR_END: system_localPeerId

  // ANCHOR: system_name
  // system.name
  const name = await api.rpc.system.name()
  console.log(name.toJSON())
  // ANCHOR_END: system_name

  // ANCHOR: system_nodeRoles
  // system.nodeRoles
  const nodeRoles = await api.rpc.system.nodeRoles()
  nodeRoles.forEach((e) => console.log(e.toString()))
  // ANCHOR_END: system_nodeRoles

  // ANCHOR: system_peers
  // system.peers
  const peers = await api.rpc.system.peers()
  peers.forEach((e) => console.log(e.toString()))
  // ANCHOR_END: system_peers

  // ANCHOR: system_properties
  // system.properties
  const properties = await api.rpc.system.properties()
  console.log(properties.isEthereum.toString())
  console.log(properties.ss58Format.toString())
  if (properties.tokenDecimals.isSome) {
    properties.tokenDecimals.value.forEach((e) => console.log(e.toString()))
  }
  if (properties.tokenSymbol.isSome) {
    properties.tokenSymbol.value.forEach((e) => console.log(e.toString()))
  }
  // ANCHOR_END: system_properties

  // ANCHOR: system_syncState
  // system.syncState
  const syncState = await api.rpc.system.syncState()
  console.log(syncState.startingBlock.toNumber())
  console.log(syncState.currentBlock.toNumber())
  if (syncState.highestBlock.isSome) {
    console.log(syncState.highestBlock.value.toNumber())
  }
  // ANCHOR_END: system_syncState

  // ANCHOR: system_version
  // system.version
  const version = await api.rpc.system.version()
  console.log(version.toString())
  // ANCHOR_END: system_version

  // ANCHOR: payment_queryInfo
  // payment.queryInfo
  const balanceTx = api.tx.balances.transferKeepAlive(
    "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
    SDK.oneAvail(),
  )
  const paymentInfo = await balanceTx.paymentInfo("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")

  console.log(paymentInfo.weight.refTime.toNumber())
  console.log(paymentInfo.weight.proofSize.toNumber())
  console.log(paymentInfo.class.type)
  console.log(paymentInfo.partialFee.toBn().toString())
  // ANCHOR_END: payment_queryInfo

  // ANCHOR: payment_queryFeeDetails
  // payment.queryFeeDetails
  const blockHash2 = await api.rpc.chain.getBlockHash()
  const nonce = await utils.getNonceNode(api, "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
  const runtimeVersion = api.runtimeVersion
  const signatureOptions = { blockHash: blockHash2, genesisHash: api.genesisHash, nonce, runtimeVersion }
  const fakeTx = balanceTx.signFake("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", signatureOptions)

  const queryFeeDetails: any = await api.call.transactionPaymentApi.queryFeeDetails(fakeTx.toHex(), null)
  const inclusionFee = {
    baseFee: queryFeeDetails.inclusionFee.__internal__raw.baseFee,
    lenFee: queryFeeDetails.inclusionFee.__internal__raw.lenFee,
    adjustedWeightFee: queryFeeDetails.inclusionFee.__internal__raw.adjustedWeightFee,
  } as InclusionFee

  console.log(inclusionFee.baseFee.toString())
  console.log(inclusionFee.lenFee.toString())
  console.log(inclusionFee.adjustedWeightFee.toString())
  // ANCHOR_END: payment_queryFeeDetails

  // ANCHOR: kate_blockLength
  // kate.blockLength
  const account = SDK.alice()
  const tx = sdk.tx.dataAvailability.submitData("My Data")
  const res = (await tx.executeWaitForFinalization(account))._unsafeUnwrap()
  const [txIndex, blockHash] = [res.txIndex, res.blockHash]

  const blockLength = await (api.rpc as any).kate.blockLength(blockHash)
  console.log(blockLength.max.normal.toNumber())
  console.log(blockLength.max.operational.toNumber())
  console.log(blockLength.max.mandatory.toNumber())
  console.log(blockLength.cols.toNumber())
  console.log(blockLength.rows.toNumber())
  console.log(blockLength.chunkSize.toNumber())
  // ANCHOR_END: kate_blockLength

  // ANCHOR: kate_queryDataProof
  // kate.queryDataProof
  const dataProof = await (api.rpc as any).kate.queryDataProof(txIndex, blockHash)
  console.log(dataProof.dataProof.roots.dataRoot.toString())
  console.log(dataProof.dataProof.roots.blobRoot.toString())
  console.log(dataProof.dataProof.roots.bridgeRoot.toString())
  dataProof.dataProof.proof.forEach((e: any) => console.log(e))
  console.log(dataProof.dataProof.numberOfLeaves.toNumber())
  console.log(dataProof.dataProof.leafIndex.toNumber())
  console.log(dataProof.dataProof.leaf.toString())
  console.log(dataProof.message.toString())
  // ANCHOR_END: kate_queryDataProof

  // ANCHOR: kate_queryProof
  // kate.queryProof
  const cell = [[0, 0]]
  const proof = await (api.rpc as any).kate.queryProof(cell, blockHash)
  proof.forEach((e: any) => e.forEach((g: any) => console.log(g.toString())))
  // ANCHOR_END: kate_queryProof

  // ANCHOR: kate_queryRows
  // kate.queryRows
  const rows = [0]
  const rowsResult = await (api.rpc as any).kate.queryRows(rows, blockHash)
  rowsResult.forEach((e: any) => e.forEach((g: any) => console.log(g.toString())))
  // ANCHOR_END: kate_queryRows
}
