import * as Basics from "./basics/index"
import * as Examples from "./examples/index"
import * as Rpc from "./rpc/index"
import * as Storage from "./storage/index"

async function main() {
  await Basics.run()
  await Examples.run()
  await Rpc.run()
  await Storage.run()
}

main()
  .catch((e) => {
    console.log(e)
    process.exit(1)
  })
  .finally(() => {
    console.log("All Good")
    process.exit(0)
  })
