macro_rules! ser_impl {
	($t:ty, $len:expr) => {
		impl ArkSimpleSerialize<$len> for $t {
			fn to_bytes(&self) -> [u8; $len] {
				use poly_multiproof::ark_serialize::CanonicalSerialize;
				let mut out = [0u8; $len];
				self.serialize_compressed(&mut out[..]).unwrap();
				out
			}
		}
	};
}
trait ArkSimpleSerialize<const N: usize> {
	const LEN: usize = N;
	fn to_bytes(&self) -> [u8; N];
}

ser_impl!(poly_multiproof::m1_blst::G1Affine, 48);
ser_impl!(poly_multiproof::m1_blst::G1, 48);
ser_impl!(poly_multiproof::m1_blst::G2Affine, 96);
ser_impl!(poly_multiproof::m1_blst::G2, 96);
ser_impl!(poly_multiproof::m1_blst::Fr, 32);

#[cfg(test)]
mod tests {
	use poly_multiproof::{
		ark_ff::UniformRand,
		m1_blst::{Fr, G1Affine, G2Affine, G1, G2},
	};
	use rand::{Rng, SeedableRng};
	use rand_chacha::ChaChaRng;

	use super::ArkSimpleSerialize;
	use crate::Seed;

	fn test_nopanic<const N: usize, T: ArkSimpleSerialize<N> + UniformRand>(rng: &mut impl Rng) {
		let p = T::rand(rng);
		p.to_bytes();
	}
	#[test]
	fn basic_nopanic() {
		let mut rng = ChaChaRng::from_seed(Seed::default());
		test_nopanic::<32, Fr>(&mut rng);
		test_nopanic::<48, G1Affine>(&mut rng);
		test_nopanic::<48, G1>(&mut rng);
		test_nopanic::<96, G2Affine>(&mut rng);
		test_nopanic::<96, G2>(&mut rng);
	}
}
