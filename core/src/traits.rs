pub mod get_app_id;
pub use get_app_id::GetAppId;

#[cfg(feature = "runtime")]
pub mod extended_header;
#[cfg(feature = "runtime")]
pub use extended_header::ExtendedHeader;

#[cfg(feature = "runtime")]
pub mod extended_block;
#[cfg(feature = "runtime")]
pub use extended_block::ExtendedBlock;

pub trait MaybeCaller<A> {
	fn caller(&self) -> Option<&A>;
}
