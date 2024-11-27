import * as Basics from "./basics/index"
import * as Examples from "./examples/index"

async function main() {
  await Examples.run()
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
