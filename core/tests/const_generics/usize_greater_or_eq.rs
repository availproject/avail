use avail_core::const_generic_asserts::USizeGreaterOrEq;

fn const_generic_ge<const N: usize, const M: usize>() {
	let () = USizeGreaterOrEq::<N, M>::OK;
}

fn main() {
	const_generic_ge::<0, 0>();
	const_generic_ge::<1, 0>();
	const_generic_ge::<1, 1>();
	const_generic_ge::<100, 99>();
	const_generic_ge::<100, 100>();
	const_generic_ge::<{ usize::MAX }, { usize::MAX - 1 }>();
	const_generic_ge::<{ usize::MAX }, { usize::MAX }>();
}
