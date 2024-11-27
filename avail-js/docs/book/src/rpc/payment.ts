import { SDK, Account, BN } from "./../../../../src/index"

const main = async () => {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api
  const alice = Account.alice(sdk)

  // Payment Query Info
  const tx = api.tx.balances.transferKeepAlive("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", SDK.oneAvail())
  const paymentInfo = await tx.paymentInfo("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")

  console.log(paymentInfo.weight.refTime.toNumber())
  console.log(paymentInfo.weight.proofSize.toNumber())
  console.log(paymentInfo.class.type)
  console.log(paymentInfo.partialFee.toBn().toString())

  // Payment Query Free Details
  const mtx2 = await alice.balanceTransfer("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", SDK.oneAvail())
  const tx2 = mtx2._unsafeUnwrap()
  const gtx2 = (await tx2.details.fetchBlock(api)).transactionByIndex(tx2.details.txIndex)._unsafeUnwrap()
  const queryFeeDetails: any = await api.call.transactionPaymentApi.queryFeeDetails(gtx2.toHex(), null)

  const baseFee: BN = queryFeeDetails.inclusionFee.__internal__raw.baseFee
  const lenFee: BN = queryFeeDetails.inclusionFee.__internal__raw.lenFee
  const adjustedWeightFee: BN = queryFeeDetails.inclusionFee.__internal__raw.adjustedWeightFee
  console.log(baseFee.toString())
  console.log(lenFee.toString())
  console.log(adjustedWeightFee.toString())

  process.exit()
}
main()
