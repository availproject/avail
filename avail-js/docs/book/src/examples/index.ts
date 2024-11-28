import * as Batch from "./batch"
import * as DA from "./data_submission"
import * as Events from "./events"
import * as Transactions from "./transactions"
import * as Validator from "./validator"

export async function run() {
  await Validator.run()
}
