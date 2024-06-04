use core::mem::size_of;

pub struct UsizeNonZero<const N: usize>;
impl<const N: usize> UsizeNonZero<N> {
	#[allow(dead_code)]
	pub const OK: () = assert!(N != 0, "must be non-zero");
}

pub struct UsizeEven<const N: usize>;
impl<const N: usize> UsizeEven<N> {
	#[allow(dead_code)]
	pub const OK: () = assert!(N % 2 == 0, "must be even");
}

pub struct USizeSafeCastToU32<const N: usize>;
impl<const N: usize> USizeSafeCastToU32<N> {
	#[allow(dead_code)]
	pub const OK: () = assert!(
		size_of::<usize>() <= size_of::<u32>() || N <= u32::MAX as usize,
		"must be safe to cast to u32"
	);
}

pub struct USizeGreaterOrEq<const N: usize, const M: usize>;
impl<const N: usize, const M: usize> USizeGreaterOrEq<N, M> {
	#[allow(dead_code)]
	pub const OK: () = assert!(N >= M, "must be greater or equal");
}

#[cfg(test)]
mod tests {
	#[test]
	fn non_zero() {
		let t = trybuild::TestCases::new();
		t.pass("tests/const_generics/usize_non_zero.rs");
		t.compile_fail("tests/const_generics/usize_non_zero_fail.rs");
	}

	#[test]
	fn even() {
		let t = trybuild::TestCases::new();
		t.pass("tests/const_generics/usize_even.rs");
		t.compile_fail("tests/const_generics/usize_even_fail.rs");
	}

	#[test]
	fn safe_cast_to_u32() {
		let t = trybuild::TestCases::new();
		t.pass("tests/const_generics/usize_safe_cast_to_u32.rs");
		t.compile_fail("tests/const_generics/usize_safe_cast_to_u32_fail.rs");
	}

	#[test]
	fn greater_or_eq() {
		let t = trybuild::TestCases::new();
		t.pass("tests/const_generics/usize_greater_or_eq.rs");
		t.compile_fail("tests/const_generics/usize_greater_or_eq_fail.rs");
	}
}
