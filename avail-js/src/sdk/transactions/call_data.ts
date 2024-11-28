import { GenericExtrinsic } from "@polkadot/types"
import { BN } from ".."

export async function getData<T>(
  tx: GenericExtrinsic,
  c: { decode(arg0: GenericExtrinsic): T | null },
): Promise<T | null> {
  return c.decode(tx)
}

export namespace DataAvailability {
  export class SubmitData {
    constructor(public data: string) {}

    static decode(tx: GenericExtrinsic): SubmitData | null {
      if (tx.method.section != "dataAvailability" || tx.method.method != "submitData") {
        return null
      }

      let dataHex = tx.method.args.map((a) => a.toString()).join(", ")
      if (dataHex.startsWith("0x")) {
        dataHex = dataHex.slice(2)
      }

      return new SubmitData(dataHex)
    }
  }
}

export namespace Balances {
  export class TransferKeepAlive {
    constructor(
      public dest: string,
      public value: BN,
    ) {}

    static decode(tx: GenericExtrinsic): TransferKeepAlive | null {
      if (tx.method.section != "balances" || tx.method.method != "transferKeepAlive") {
        return null
      }
      const args = tx.method.args as any
      const dest = args[0].toString()
      const value = new BN(args[1].toString())

      return new TransferKeepAlive(dest, value)
    }
  }
}
