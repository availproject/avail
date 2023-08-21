// MIT License

// Copyright (c) 2022 Bright Inventions

// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use crate::verify::VerificationError::InvalidVerificationKey;
use bls12_381::{Bls12, G1Affine, G2Affine, Scalar};
use group::{prime::PrimeCurveAffine, Curve};
use pairing::{Engine, MultiMillerLoop};
use sp_std::{ops::AddAssign, prelude::*};

pub const SUPPORTED_CURVE: &str = "bls12381";
pub const SUPPORTED_PROTOCOL: &str = "groth16";

/// Stores G1 field points (x, y) in an array.
/// positions `[0,48]` contains x
/// positions `[49, 96]` contains y
pub struct G1UncompressedBytes {
	inner: [u8; 96],
}

/// Stores G2 field points (x, y) in an array.
/// positions `[0,48]` contains x_c1
/// positions `[49, 96]` contains x_c0
/// positions `[97,144]` contains y_c1
/// positions `[145, 192]` contains y_c0
pub struct G2UncompressedBytes {
	inner: [u8; 192],
}

impl G1UncompressedBytes {
	pub fn new(x: [u8; 48], y: [u8; 48]) -> Self {
		let mut new_bytes: [u8; 96] = [0; 96];

		new_bytes[..48].copy_from_slice(&x[..48]);
		new_bytes[48..(48 + 48)].copy_from_slice(&y[..48]);
		new_bytes[0] |= &0u8;

		G1UncompressedBytes { inner: new_bytes }
	}
}

impl G2UncompressedBytes {
	pub fn new(x_c0: [u8; 48], x_c1: [u8; 48], y_c0: [u8; 48], y_c1: [u8; 48]) -> Self {
		let mut new_bytes: [u8; 192] = [0; 192];

		new_bytes[..48].copy_from_slice(&x_c1[..48]);
		new_bytes[48..(48 + 48)].copy_from_slice(&x_c0[..48]);
		new_bytes[96..(48 + 96)].copy_from_slice(&y_c1[..48]);
		new_bytes[144..(48 + 144)].copy_from_slice(&y_c0[..48]);

		new_bytes[0] |= &0u8;

		G2UncompressedBytes { inner: new_bytes }
	}
}

impl TryFrom<&G1UncompressedBytes> for G1Affine {
	type Error = ();

	fn try_from(value: &G1UncompressedBytes) -> Result<Self, Self::Error> {
		let g1 = G1Affine::from_uncompressed(&value.inner);
		if g1.is_none().into() {
			Err(())
		} else {
			Ok(g1.unwrap())
		}
	}
}

impl TryFrom<&G2UncompressedBytes> for G2Affine {
	type Error = ();

	fn try_from(value: &G2UncompressedBytes) -> Result<Self, Self::Error> {
		let g2 = G2Affine::from_uncompressed(&value.inner);
		if g2.is_none().into() {
			Err(())
		} else {
			Ok(g2.unwrap())
		}
	}
}

/// Represents Groth16 verification key
pub struct VerificationKey {
	pub alpha: G1Affine,
	pub beta: G2Affine,
	pub gamma: G2Affine,
	pub delta: G2Affine,
	pub ic: Vec<G1Affine>,
}

#[derive(Debug)]
pub enum VerificationKeyCreationError {
	PointCreationError,
}

impl VerificationKey {
	pub fn from_uncompressed(
		alpha: &G1UncompressedBytes,
		beta: &G2UncompressedBytes,
		gamma: &G2UncompressedBytes,
		delta: &G2UncompressedBytes,
		ic: &Vec<G1UncompressedBytes>,
	) -> Result<Self, VerificationKeyCreationError> {
		let alpha = alpha
			.try_into()
			.map_err(|_| VerificationKeyCreationError::PointCreationError)?;
		let beta: G2Affine = beta
			.try_into()
			.map_err(|_| VerificationKeyCreationError::PointCreationError)?;
		let gamma: G2Affine = gamma
			.try_into()
			.map_err(|_| VerificationKeyCreationError::PointCreationError)?;
		let delta: G2Affine = delta
			.try_into()
			.map_err(|_| VerificationKeyCreationError::PointCreationError)?;
		let mut ic_2: Vec<G1Affine> = Vec::with_capacity(ic.len());

		for i in ic {
			ic_2.push(
				G1Affine::try_from(i)
					.map_err(|_| VerificationKeyCreationError::PointCreationError)?,
			);
		}

		Ok(VerificationKey {
			alpha,
			beta,
			gamma,
			delta,
			ic: ic_2,
		})
	}
}

/// Represents Groth16 proof
pub struct GProof {
	pub a: G1Affine,
	pub b: G2Affine,
	pub c: G1Affine,
}

#[derive(Debug)]
pub enum GProofCreationError {
	PointCreationError,
}

impl GProof {
	pub fn from_uncompressed(
		a: &G1UncompressedBytes,
		b: &G2UncompressedBytes,
		c: &G1UncompressedBytes,
	) -> Result<Self, GProofCreationError> {
		let a = a
			.try_into()
			.map_err(|_| GProofCreationError::PointCreationError)?;
		let b = b
			.try_into()
			.map_err(|_| GProofCreationError::PointCreationError)?;
		let c = c
			.try_into()
			.map_err(|_| GProofCreationError::PointCreationError)?;

		Ok(GProof { a, b, c })
	}
}

#[derive(Debug, PartialEq)]
pub enum VerificationError {
	InvalidVerificationKey,
}

pub type VerificationResult = Result<bool, VerificationError>;

pub type PublicInputs = Vec<Scalar>;

/// Turns `u64` values into `Scalar` representation
pub fn prepare_public_inputs(inputs: Vec<u64>) -> Vec<Scalar> {
	inputs.into_iter().map(Scalar::from).collect()
}

/// Verifies given proof with given verification key and public inputs
pub fn verify(vk: VerificationKey, proof: GProof, inputs: PublicInputs) -> VerificationResult {
	let public_inputs: &[<Bls12 as Engine>::Fr] = &inputs;

	if (public_inputs.len() + 1) != vk.ic.len() {
		return Err(InvalidVerificationKey);
	}

	// ic contains Lᵢ(τ)/δ
	// Lᵢ(x) = β * Aᵢ(x) + α * Bᵢ(x) + Cᵢ(x)
	// public variables [33]
	// w = [1, 33, ...private variables]
	// acc contains sum of Lᵢ(x) * wᵢ
	let mut acc = vk.ic[0].to_curve();
	for (i, b) in public_inputs.iter().zip(vk.ic.iter().skip(1)) {
		AddAssign::<&<Bls12 as Engine>::G1>::add_assign(&mut acc, &(*b * i));
	}

	//lhs
	// Aₚ*Bₚ
	let a_b_pairing = Bls12::pairing(&proof.a, &proof.b);

	//rhs
	// αβ + (L_input(τ)/γ)γ + Cₚδ
	let final_result = Bls12::multi_miller_loop(&[
		(&vk.alpha, &vk.beta.into()),
		(&acc.to_affine(), &vk.gamma.into()),
		(&proof.c, &vk.delta.into()),
	])
	.final_exponentiation();

	Ok(a_b_pairing == final_result)
}

#[cfg(test)]
mod tests {
	use crate::verify::{
		verify, G1UncompressedBytes, G2UncompressedBytes, GProof, VerificationError,
		VerificationKey,
	};
	use bls12_381::{G1Affine, G2Affine};

	const ALPHA_X: &str = "2417420058161902631695569321985275527817337553240735969068630412919230058600548397578577183742111992841943587142680";
	const ALPHA_Y: &str = "2683193963041639430431668252069589353703764749562535314981925385889474793061455502785968498855669710056680025802535";

	const BETA_X_C0: &str = "2953983861911780746898420772852203750596202163211813473761616529894571940032171065334774419373056700627707738200018";
	const BETA_X_C1: &str = "3062465588861097636655055190501059315624734570742089309263797407021640154269222765149244340402777629537231482465213";

	const BETA_Y_C0: &str = "2880510548434910442614869111285946610418075557776097505115113030863387119802265689270335925248001883102867749676243";
	const BETA_Y_C1: &str = "2872114062532568575643729173452461066994643453813848213872870173636132169046691827766994227240293333106164659529444";

	const GAMMA_X_C0: &str = "352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160";
	const GAMMA_X_C1: &str = "3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758";

	const GAMMA_Y_C0: &str = "1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905";
	const GAMMA_Y_C1: &str = "927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582";

	const DELTA_X_C0: &str = "1397400294785329269149248027941029918234275798984995986592789994215372037046682288247459925132482655775231958770596";
	const DELTA_X_C1: &str = "3613651892030917982825314322568444757238870140073427833524931882395488683192849483836696311878674061447378155414322";

	const DELTA_Y_C0: &str = "1454420022135097547429203607513890428221900276713697693498600894391966225725356692084173923746366083520797626734711";
	const DELTA_Y_C1: &str = "2405306655262521121779739123612338596090750073099847349336699337941746231436397110773618181083856700942862129820841";

	const IC_1_X: &str = "1036455169342233390855996586834520647962171510914420928779905953251272176363349160512017514969413843826714495861777";
	const IC_1_Y: &str = "3225757548975669202743314017707154170140342810479555354528303455797434256089415962868447574306245203533729979725838";

	const IC_2_X: &str = "2306767568146465899824632338747274961711075325739057886746993285987967410538122442295923393427774655394152050218360";
	const IC_2_Y: &str = "1110686736735022843500989850943596336256510944040379817126812118843722981304262779720098389756327870602977197635083";

	const PI_A_X: &str = "1547868284561670884744470829066291861753711715427536197016979117727657722537367306855408779073400007356480755992286";
	const PI_A_Y: &str = "133377702143528739575377729631360601614088262416333931136172973337607317017542609318946667454426700160620492918070";

	const PI_B_X_C0: &str = "3464179927623990666132434581669710292812271436336621246126774308069940684644800766694467705159555008883836001203558";
	const PI_B_X_C1: &str = "2546213637341159614042232103352468058136925633034122854640067781563520449770334670597953179425897845578304324932654";
	const PI_B_Y_C0: &str = "1727172519477219519750367293438016239792036515829871417520013243406611034907195588907593103368826194109213319586533";
	const PI_B_Y_C1: &str = "1608709552654556864133663038831358765687167633553533833302139692670076873672935498325809703404354703063813928303923";

	const PI_C_X: &str = "1754096103716358561952826128249523421393931227029702817784288419733418512708632119712049074095306383315056978720954";
	const PI_C_Y: &str = "2834250288052560472935431224341595955480629006732618887386362957441961005785403404522081920080207211610068590548972";

	construct_uint! {
		pub struct U256(6);
	}

	#[test]
	fn verification_key_from_correct_coordinates_is_ok() {
		let vk = VerificationKey::from_uncompressed(
			&G1UncompressedBytes::new(from_dec_string(ALPHA_X), from_dec_string(ALPHA_Y)),
			&G2UncompressedBytes::new(
				from_dec_string(BETA_X_C0),
				from_dec_string(BETA_X_C1),
				from_dec_string(BETA_Y_C0),
				from_dec_string(BETA_Y_C1),
			),
			&G2UncompressedBytes::new(
				from_dec_string(GAMMA_X_C0),
				from_dec_string(GAMMA_X_C1),
				from_dec_string(GAMMA_Y_C0),
				from_dec_string(GAMMA_Y_C1),
			),
			&G2UncompressedBytes::new(
				from_dec_string(DELTA_X_C0),
				from_dec_string(DELTA_X_C1),
				from_dec_string(DELTA_Y_C0),
				from_dec_string(DELTA_Y_C1),
			),
			&vec![
				G1UncompressedBytes::new(from_dec_string(IC_1_X), from_dec_string(IC_1_Y)),
				G1UncompressedBytes::new(from_dec_string(IC_2_X), from_dec_string(IC_2_Y)),
			],
		);
		assert!(vk.is_ok())
	}

	#[test]
	fn proof_from_correct_coordinates_is_ok() {
		let proof = GProof::from_uncompressed(
			&G1UncompressedBytes::new(from_dec_string(PI_A_X), from_dec_string(PI_A_Y)),
			&G2UncompressedBytes::new(
				from_dec_string(PI_B_X_C0),
				from_dec_string(PI_B_X_C1),
				from_dec_string(PI_B_Y_C0),
				from_dec_string(PI_B_Y_C1),
			),
			&G1UncompressedBytes::new(from_dec_string(PI_C_X), from_dec_string(PI_C_Y)),
		);
		assert!(proof.is_ok())
	}

	#[test]
	fn verify_correct_proof() {
		// circuit description https://github.com/iden3/circom/blob/7e59274c3e78674c2178766f9b8a4371c760ac3a/mkdocs/docs/getting-started/writing-circuits.md

		//----------VK------------//
		// blog/data/verification_key.json
		let alpha: G1Affine =
			(&G1UncompressedBytes::new(from_dec_string(ALPHA_X), from_dec_string(ALPHA_Y)))
				.try_into()
				.unwrap();

		let beta: G2Affine = (&G2UncompressedBytes::new(
			from_dec_string(BETA_X_C0),
			from_dec_string(BETA_X_C1),
			from_dec_string(BETA_Y_C0),
			from_dec_string(BETA_Y_C1),
		))
			.try_into()
			.unwrap();

		let gamma: G2Affine = (&G2UncompressedBytes::new(
			from_dec_string(GAMMA_X_C0),
			from_dec_string(GAMMA_X_C1),
			from_dec_string(GAMMA_Y_C0),
			from_dec_string(GAMMA_Y_C1),
		))
			.try_into()
			.unwrap();
		let delta: G2Affine = (&G2UncompressedBytes::new(
			from_dec_string(DELTA_X_C0),
			from_dec_string(DELTA_X_C1),
			from_dec_string(DELTA_Y_C0),
			from_dec_string(DELTA_Y_C1),
		))
			.try_into()
			.unwrap();

		let ic_1: G1Affine =
			(&G1UncompressedBytes::new(from_dec_string(IC_1_X), from_dec_string(IC_1_Y)))
				.try_into()
				.unwrap();
		let ic_2: G1Affine =
			(&G1UncompressedBytes::new(from_dec_string(IC_2_X), from_dec_string(IC_2_Y)))
				.try_into()
				.unwrap();

		//----------END OF VK------------//

		//----------PROOF---------------//
		// blog/data/proof.json
		let pi_a: G1Affine =
			(&G1UncompressedBytes::new(from_dec_string(PI_A_X), from_dec_string(PI_A_Y)))
				.try_into()
				.unwrap();
		let pi_b: G2Affine = (&G2UncompressedBytes::new(
			from_dec_string(PI_B_X_C0),
			from_dec_string(PI_B_X_C1),
			from_dec_string(PI_B_Y_C0),
			from_dec_string(PI_B_Y_C1),
		))
			.try_into()
			.unwrap();
		let pi_c: G1Affine =
			(&G1UncompressedBytes::new(from_dec_string(PI_C_X), from_dec_string(PI_C_Y)))
				.try_into()
				.unwrap();
		//------END OF PROOF-----------//

		//----------VERIFICATION---------------//
		assert!(verify(
			VerificationKey {
				alpha,
				beta,
				gamma,
				delta,
				ic: vec![ic_1, ic_2]
			},
			GProof {
				a: pi_a,
				b: pi_b,
				c: pi_c
			},
			// blog/data/public.json
			[12.into()].into(),
		)
		.unwrap())
		//--------END OF VERIFICATION---------//
	}

	#[test]
	fn verify_incorrect_proof() {
		//----------VK------------//
		// sample/verification_key.json
		let alpha: G1Affine =
			(&G1UncompressedBytes::new(from_dec_string(ALPHA_X), from_dec_string(ALPHA_Y)))
				.try_into()
				.unwrap();

		let beta: G2Affine = (&G2UncompressedBytes::new(
			from_dec_string(BETA_X_C0),
			from_dec_string(BETA_X_C1),
			from_dec_string(BETA_Y_C0),
			from_dec_string(BETA_Y_C1),
		))
			.try_into()
			.unwrap();

		let gamma: G2Affine = (&G2UncompressedBytes::new(
			from_dec_string(GAMMA_X_C0),
			from_dec_string(GAMMA_X_C1),
			from_dec_string(GAMMA_Y_C0),
			from_dec_string(GAMMA_Y_C1),
		))
			.try_into()
			.unwrap();
		let delta: G2Affine = (&G2UncompressedBytes::new(
			from_dec_string(DELTA_X_C0),
			from_dec_string(DELTA_X_C1),
			from_dec_string(DELTA_Y_C0),
			from_dec_string(DELTA_Y_C1),
		))
			.try_into()
			.unwrap();

		let ic_1: G1Affine =
			(&G1UncompressedBytes::new(from_dec_string(IC_1_X), from_dec_string(IC_1_Y)))
				.try_into()
				.unwrap();
		let ic_2: G1Affine =
			(&G1UncompressedBytes::new(from_dec_string(IC_2_X), from_dec_string(IC_2_Y)))
				.try_into()
				.unwrap();

		//----------END OF VK------------//

		//----------PROOF---------------//
		// sample/proof.json
		let pi_c: G1Affine =
			(&G1UncompressedBytes::new(from_dec_string(PI_A_X), from_dec_string(PI_A_Y)))
				.try_into()
				.unwrap();
		let pi_b: G2Affine = (&G2UncompressedBytes::new(
			from_dec_string(PI_B_X_C0),
			from_dec_string(PI_B_X_C1),
			from_dec_string(PI_B_Y_C0),
			from_dec_string(PI_B_Y_C1),
		))
			.try_into()
			.unwrap();
		let pi_a: G1Affine =
			(&G1UncompressedBytes::new(from_dec_string(PI_C_X), from_dec_string(PI_C_Y)))
				.try_into()
				.unwrap();
		//------END OF PROOF-----------//

		//----------VERIFICATION---------------//
		assert!(!verify(
			VerificationKey {
				alpha,
				beta,
				gamma,
				delta,
				ic: vec![ic_1, ic_2]
			},
			GProof {
				a: pi_a,
				b: pi_b,
				c: pi_c
			},
			[33.into()].into(),
		)
		.unwrap())
		//--------END OF VERIFICATION---------//
	}

	#[test]
	fn verify_with_incorrect_ic_len() {
		//----------VK------------//
		// sample/verification_key.json
		let alpha: G1Affine =
			(&G1UncompressedBytes::new(from_dec_string(ALPHA_X), from_dec_string(ALPHA_Y)))
				.try_into()
				.unwrap();

		let beta: G2Affine = (&G2UncompressedBytes::new(
			from_dec_string(BETA_X_C0),
			from_dec_string(BETA_X_C1),
			from_dec_string(BETA_Y_C0),
			from_dec_string(BETA_Y_C1),
		))
			.try_into()
			.unwrap();

		let gamma: G2Affine = (&G2UncompressedBytes::new(
			from_dec_string(GAMMA_X_C0),
			from_dec_string(GAMMA_X_C1),
			from_dec_string(GAMMA_Y_C0),
			from_dec_string(GAMMA_Y_C1),
		))
			.try_into()
			.unwrap();
		let delta: G2Affine = (&G2UncompressedBytes::new(
			from_dec_string(DELTA_X_C0),
			from_dec_string(DELTA_X_C1),
			from_dec_string(DELTA_Y_C0),
			from_dec_string(DELTA_Y_C1),
		))
			.try_into()
			.unwrap();

		let ic_1: G1Affine =
			(&G1UncompressedBytes::new(from_dec_string(IC_1_X), from_dec_string(IC_1_Y)))
				.try_into()
				.unwrap();

		//----------END OF VK------------//

		//----------PROOF---------------//
		// sample/proof.json
		let pi_a: G1Affine =
			(&G1UncompressedBytes::new(from_dec_string(PI_A_X), from_dec_string(PI_A_Y)))
				.try_into()
				.unwrap();
		let pi_b: G2Affine = (&G2UncompressedBytes::new(
			from_dec_string(PI_B_X_C0),
			from_dec_string(PI_B_X_C1),
			from_dec_string(PI_B_Y_C0),
			from_dec_string(PI_B_Y_C1),
		))
			.try_into()
			.unwrap();
		let pi_c: G1Affine =
			(&G1UncompressedBytes::new(from_dec_string(PI_C_X), from_dec_string(PI_C_Y)))
				.try_into()
				.unwrap();
		//------END OF PROOF-----------//

		//----------VERIFICATION---------------//
		assert_eq!(
			verify(
				VerificationKey {
					alpha,
					beta,
					gamma,
					delta,
					ic: vec![ic_1]
				},
				GProof {
					a: pi_a,
					b: pi_b,
					c: pi_c
				},
				[33.into()].into(),
			)
			.err()
			.unwrap(),
			VerificationError::InvalidVerificationKey
		)
		//--------END OF VERIFICATION---------//
	}

	fn from_dec_string(number: &str) -> [u8; 48] {
		let mut bytes: [u8; 48] = [0; 48];
		U256::from_dec_str(number)
			.unwrap()
			.to_big_endian(bytes.as_mut_slice());
		bytes
	}
}
