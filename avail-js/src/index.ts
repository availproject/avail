// Allows for generic imports (eg. import {...} from 'avail-js-sdk')
export * from "./chain"
export * from "./helpers"
export * from "./spec"

// Allows for custom imports (eg. import {...} from 'avail-js-sdk/chain')
export * as chain from "./chain"
export * as helpers from "./helpers"
export * as spec from "./spec"

// Export the SDK classes
export * from "./sdk"

// Re-exports to avoid duplication
export * from "@polkadot/api"
export * as polkadotApi from "@polkadot/api"
export * as polkadotUtil from "@polkadot/util"
