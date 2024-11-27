import * as H256Example from "./h256"
import * as Nonce from "./nonce"

export async function run() {
  await H256Example.run()
  await Nonce.run()
}
