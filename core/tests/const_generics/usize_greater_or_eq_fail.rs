use avail_core::const_generic_asserts::USizeGreaterOrEq;

fn const_generic_ge<const N: usize, const M: usize> () {
	let () = USizeGreaterOrEq::<N,M>::OK;
}

fn main () {
	const_generic_ge::<1, 2>();
}
