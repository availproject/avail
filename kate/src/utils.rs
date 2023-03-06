macro_rules! ser_impl {
	($t:ty, $len:expr) => {
		impl ArkSimpleSerialize<$len> for $t {
			fn to_bytes(&self) -> [u8; $len] {
				use ark_serialize::CanonicalSerialize;
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

ser_impl!(ark_bls12_381::G1Affine, 48);
ser_impl!(ark_bls12_381::G1Projective, 48);
ser_impl!(ark_bls12_381::G2Affine, 96);
ser_impl!(ark_bls12_381::G2Projective, 96);
ser_impl!(ark_bls12_381::Fr, 32);

#[cfg(test)]
mod tests {
	use ark_bls12_381::*;
	use ark_ff::UniformRand;
	use rand::{Rng, SeedableRng};
	use rand_chacha::ChaChaRng;

	use crate::Seed;
	use super::ArkSimpleSerialize;

	fn test_nopanic<const N: usize, T: ArkSimpleSerialize<N> + UniformRand>(rng: &mut impl Rng) {
		let p = T::rand(rng);
		p.to_bytes();
	}
	#[test]
	fn basic_nopanic() {
		let mut rng = ChaChaRng::from_seed(Seed::default());
        test_nopanic::<32, Fr>(&mut rng);
		test_nopanic::<48, G1Affine>(&mut rng);
		test_nopanic::<48, G1Projective>(&mut rng);
		test_nopanic::<96, G2Affine>(&mut rng);
		test_nopanic::<96, G2Projective>(&mut rng);
	}
}
