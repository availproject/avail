use crate::traits::ExtendedHeader;
use sp_runtime::traits::Block;

/// Extended Block trait that extends substrate primitive Block to include ExtendedHeader in the header
pub trait ExtendedBlock: Block<Header = Self::ExtHeader> {
	type ExtHeader: ExtendedHeader;
}
