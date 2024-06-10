use avail_core::const_generic_asserts::USizeSafeCastToU32;

fn const_generic_safe_cast_to_u32<const N: usize>() {
	let () = USizeSafeCastToU32::<N>::OK;
}

fn main() {
	const_generic_safe_cast_to_u32::<0>();
	const_generic_safe_cast_to_u32::<{ (u32::MAX - 1) as usize }>();
	const_generic_safe_cast_to_u32::<{ u32::MAX as usize }>();
}
