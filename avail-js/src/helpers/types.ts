import type { Compact, u8, u16, u32, Vec, Enum, Struct } from "@polkadot/types-codec"
import type { Hash, Header } from "@polkadot/types/interfaces/runtime"

export interface AppId extends Compact<u32> {}
export interface DataLookupItem extends Struct {
  readonly appId: AppId
  readonly start: Compact<u32>
}
export interface DataLookupItdddm extends Struct {
  readonly appId: AppId
  readonly start: Compact<u32>
}
export interface CompactDataLookup {
  readonly size: Compact<u32>
  readonly index: Vec<DataLookupItem>
}
export interface KateCommitment extends Struct {
  readonly rows: Compact<u16>
  readonly cols: Compact<u16>
  readonly commitment: Vec<u8>
  readonly dataRoot: Hash
}
export interface V3HeaderExtension extends Struct {
  readonly appLookup: CompactDataLookup
  readonly commitment: KateCommitment
}
export interface HeaderExtension extends Enum {
  readonly isV3: boolean
  readonly asV3: V3HeaderExtension
}
export interface DaHeader extends Header {
  readonly extension: HeaderExtension
}
