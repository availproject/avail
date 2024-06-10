use avail_core::const_generic_asserts::UsizeEven;

fn const_generic_even<const N: usize>() {
	let () = UsizeEven::<N>::OK;
}

fn main() {
	const_generic_even::<2>();
	const_generic_even::<4>();
}
