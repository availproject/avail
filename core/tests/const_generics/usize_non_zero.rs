use avail_core::const_generic_asserts::UsizeNonZero;

fn const_generic_non_zero<const N: usize>() {
	let () = UsizeNonZero::<N>::OK;
}

fn main() {
	const_generic_non_zero::<1>();
	const_generic_non_zero::<2>();
}
