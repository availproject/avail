/// The example showcases how to do DA attestation.
///
/// Submits DA transaction to Avail and waits for the block to be finalized.
/// When block with DA transaction is finalized it waits for the range commitment to be included on the Ethereum,
/// once included, it fetches the proof and calls contract to verify the data blob inclusion.
///

import { ApiPromise, getKeyringFromSeed, initialize } from "avail-js-sdk"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"

import config from "../../config"
import { AddressOrPair } from "@polkadot/api-base/types/submittable"
import { ethers } from "ethers"
import abi from "../abi/availbridge.json"

/**
 *  ProofData represents a response from the api that holds proof for
 *  the blob verification.
 */
class ProofData {
  dataRootProof: Array<string>
  leafProof: Array<string>
  rangeHash: string
  dataRootIndex: number
  blobRoot: string
  bridgeRoot: string
  leaf: string
  leafIndex: number

  constructor(
    dataRootProof: Array<string>,
    leafProof: Array<string>,
    rangeHash: string,
    dataRootIndex: number,
    blobRoot: string,
    bridgeRoot: string,
    leaf: string,
    leafIndex: number,
  ) {
    this.dataRootProof = dataRootProof
    this.leafProof = leafProof
    this.rangeHash = rangeHash
    this.dataRootIndex = dataRootIndex
    this.blobRoot = blobRoot
    this.bridgeRoot = bridgeRoot
    this.leaf = leaf
    this.leafIndex = leafIndex
  }
}

/**
 * Submitting DA extrinsic to Avail.
 *
 * @param availApi api instance
 * @param data payload to send
 * @param account that is sending transaction
 * @returns {Promise<unknown>}
 */
async function submitData(availApi: ApiPromise, data: string, account: AddressOrPair) {
  return await new Promise<ISubmittableResult>((res) => {
    console.log("Sending transaction...")
    availApi.tx.dataAvailability.submitData(data).signAndSend(account, { nonce: -1 }, (result: ISubmittableResult) => {
      console.log(`Tx status: ${result.status}`)
      if (result.isError) {
        console.log(`Tx failed!`)
        res(result)
      }
      if (result.isInBlock) {
        console.log("Transaction in block, waiting for block finalization...")
      }
      if (result.isFinalized) {
        console.log(`Tx finalized.`)
        res(result)
      }
    })
  })
}

const main = async () => {
  try {
    const api = await initialize(config.endpoint)
    const account = getKeyringFromSeed(config.seed)
    const result = await submitData(api, config.data, account)
    if (result.isFinalized) {
      console.log(
        `DA transaction in finalized block: ${result.status.asFinalized}, transaction index: ${result.txIndex}`,
      )
    }

    // wait until the chain head on the Ethereum network is updated with the block range
    // in which the Avail DA transaction is included.
    while (true) {
      const getHeadRsp = await fetch(config.bridgeApi + "/avl/head")
      if (getHeadRsp.status != 200) {
        console.log("Something went wrong fetching the head.")
        break
      }
      const headRsp = await getHeadRsp.json()
      const lastCommittedBlock: number = headRsp.data.end

      const signedBlock = await api.rpc.chain.getBlock(result.status.asFinalized)
      const blockNumber: number = signedBlock.block.header.number.toNumber()

      if (lastCommittedBlock >= blockNumber) {
        console.log("Fetching the blob proof.")
        const proofResponse = await fetch(
          config.bridgeApi + "/eth/proof/" + result.status.asFinalized + "?index=" + result.txIndex,
        )
        if (proofResponse.status != 200) {
          console.log("Something went wrong fetching the proof.")
          console.log(proofResponse)
          break
        }
        const proof: ProofData = await proofResponse.json()
        console.log("Proof fetched:")
        console.log(proof)
        // call the deployed contract verification function with the inclusion proof.
        const provider = ethers.getDefaultProvider(config.ethProviderUrl)
        const contractInstance = new ethers.Contract(config.bridgeContractAddress, abi, provider)
        const isVerified = await contractInstance.verifyBlobLeaf([
          proof.dataRootProof,
          proof.leafProof,
          proof.rangeHash,
          proof.dataRootIndex,
          proof.blobRoot,
          proof.bridgeRoot,
          proof.leaf,
          proof.leafIndex,
        ])
        console.log(`Blob validation is: ${isVerified}`)
        break
      }

      console.log("Waiting to bridge inclusion commitment. This can take a while...")
      // wait for some time and check again
      await new Promise((f) => setTimeout(f, 60 * 1000))
    }

    process.exit(0)
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()
