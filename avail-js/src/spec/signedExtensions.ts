import type { ExtDef } from "@polkadot/types/extrinsic/signedExtensions/types.js"

export const signedExtensions: ExtDef = {
  CheckAppId: {
    extrinsic: {
      appId: "AppId",
    },
    payload: {},
  },
}
