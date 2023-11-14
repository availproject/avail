use ark_bn254::{Bn254, Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_groth16::{prepare_verifying_key, verify_proof, Proof, VerifyingKey};
use ark_std::boxed::Box;
use ark_std::str::FromStr;
use ark_std::string::String;
use ark_std::string::ToString;
use ark_std::vec;
use ark_std::vec::Vec;
use codec::{Decode, Encode};
use ethabi::ParamType;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_core::{H256, U256};

use crate::state::{CircomProof, PublicSignals};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub enum VerificationError {
	InvalidProof,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub struct Verifier {
	pub vk_json: VerifyingKeyJson,
}

#[derive(Debug)]
pub enum VKeyDeserializationError {
	SerdeError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub struct VerifyingKeyJson {
	#[serde(rename = "IC")]
	pub ic: Vec<Vec<String>>,

	#[serde(rename = "nPublic")]
	pub inputs_count: u32,
	pub vk_alpha_1: Vec<String>,
	pub vk_beta_2: Vec<Vec<String>>,
	pub vk_gamma_2: Vec<Vec<String>>,
	pub vk_delta_2: Vec<Vec<String>>,
	pub vk_alphabeta_12: Vec<Vec<Vec<String>>>,
	pub curve: String,
	pub protocol: String,
}

impl VerifyingKeyJson {
	pub fn to_verifying_key(self) -> VerifyingKey<Bn254> {
		let alpha_g1 = G1Affine::from(G1Projective::new(
			str_to_fq(&self.vk_alpha_1[0]),
			str_to_fq(&self.vk_alpha_1[1]),
			str_to_fq(&self.vk_alpha_1[2]),
		));
		let beta_g2 = G2Affine::from(G2Projective::new(
			// x
			Fq2::new(
				str_to_fq(&self.vk_beta_2[0][0]),
				str_to_fq(&self.vk_beta_2[0][1]),
			),
			// y
			Fq2::new(
				str_to_fq(&self.vk_beta_2[1][0]),
				str_to_fq(&self.vk_beta_2[1][1]),
			),
			// z,
			Fq2::new(
				str_to_fq(&self.vk_beta_2[2][0]),
				str_to_fq(&self.vk_beta_2[2][1]),
			),
		));

		let gamma_g2 = G2Affine::from(G2Projective::new(
			// x
			Fq2::new(
				str_to_fq(&self.vk_gamma_2[0][0]),
				str_to_fq(&self.vk_gamma_2[0][1]),
			),
			// y
			Fq2::new(
				str_to_fq(&self.vk_gamma_2[1][0]),
				str_to_fq(&self.vk_gamma_2[1][1]),
			),
			// z,
			Fq2::new(
				str_to_fq(&self.vk_gamma_2[2][0]),
				str_to_fq(&self.vk_gamma_2[2][1]),
			),
		));

		let delta_g2 = G2Affine::from(G2Projective::new(
			// x
			Fq2::new(
				str_to_fq(&self.vk_delta_2[0][0]),
				str_to_fq(&self.vk_delta_2[0][1]),
			),
			// y
			Fq2::new(
				str_to_fq(&self.vk_delta_2[1][0]),
				str_to_fq(&self.vk_delta_2[1][1]),
			),
			// z,
			Fq2::new(
				str_to_fq(&self.vk_delta_2[2][0]),
				str_to_fq(&self.vk_delta_2[2][1]),
			),
		));

		let gamma_abc_g1: Vec<G1Affine> = self
			.ic
			.iter()
			.map(|coords| {
				G1Affine::from(G1Projective::new(
					str_to_fq(&coords[0]),
					str_to_fq(&coords[1]),
					str_to_fq(&coords[2]),
				))
			})
			.collect();

		VerifyingKey::<Bn254> {
			alpha_g1,
			beta_g2,
			gamma_g2,
			delta_g2,
			gamma_abc_g1,
		}
	}
}

pub fn str_to_fq(s: &str) -> Fq {
	Fq::from_str(s).unwrap()
}

impl Verifier {
	/// Creates `Verifier` from json representation
	pub fn from_json_u8_slice(slice: &[u8]) -> Result<Self, VKeyDeserializationError> {
		serde_json::from_slice(slice).map_err(|_| VKeyDeserializationError::SerdeError)
	}

	// Verifies input based on the supplied proof and hashes
	pub fn verify(
		self,
		input_hash: H256,
		output_hash: H256,
		proof: Vec<u8>,
	) -> Result<bool, VerificationError> {
		// remove first 3 bits from input_hash and output_hash
		let bits_mask = 0b00011111;
		let mut input_swap = input_hash.to_fixed_bytes();
		let input_hash_byte_swap = input_hash[0] & bits_mask;
		input_swap[0] = input_hash_byte_swap;

		let mut output_swap = output_hash.to_fixed_bytes();
		let output_hash_byte_swap = output_hash[0] & bits_mask;
		output_swap[0] = output_hash_byte_swap;

		let decoded: (Vec<String>, Vec<Vec<String>>, Vec<String>) = decode_proof(proof);
		// TODO remove printlns
		// println!("decoded proof: {:?}", decoded);

		let circom_proof = CircomProof::new(decoded.0, decoded.1, decoded.2);
		let proof = circom_proof.to_proof();

		let mut input = vec!["0".to_string(); 2];
		input[0] = U256::from_big_endian(output_swap.as_slice()).to_string();
		input[1] = U256::from_big_endian(input_swap.as_slice()).to_string();

		let public_signals = PublicSignals::from(input);

		// println!("public signals: {:?}", public_signals);

		let result = self.verify_proof(proof.clone(), &public_signals.get());

		result.map_err(|_| VerificationError::InvalidProof)
	}
	fn verify_proof(self, proof: Proof<Bn254>, inputs: &[Fr]) -> Result<bool, VerificationError> {
		let vk = self.vk_json.to_verifying_key();
		let pvk = prepare_verifying_key(&vk);

		let result = verify_proof(&pvk, &proof, inputs);
		result.map_err(|_| VerificationError::InvalidProof)
	}
}

pub fn decode_proof(proof: Vec<u8>) -> (Vec<String>, Vec<Vec<String>>, Vec<String>) {
	let decoded = ethabi::decode(
		&[ParamType::Tuple(vec![
			ParamType::FixedArray(Box::new(ParamType::Uint(256)), 2),
			ParamType::FixedArray(
				Box::new(ParamType::FixedArray(Box::new(ParamType::Uint(256)), 2)),
				2,
			),
			ParamType::FixedArray(Box::new(ParamType::Uint(256)), 2),
		])],
		&proof,
	)
	.expect("Proof must be decodable .qed");

	let mut a0: String = String::new();
	let mut a1: String = String::new();

	let mut b00: String = String::new();
	let mut b01: String = String::new();
	let mut b10: String = String::new();
	let mut b11: String = String::new();

	let mut c0: String = String::new();
	let mut c1: String = String::new();

	if let Some(ethabi::Token::Tuple(t)) = decoded.get(0) {
		if let ethabi::Token::FixedArray(ar) = &t[0] {
			if let ethabi::Token::Uint(u) = &ar[0] {
				a0 = u.to_string();
			}
			if let ethabi::Token::Uint(u) = &ar[1] {
				a1 = u.to_string();
			}
		}

		if let ethabi::Token::FixedArray(ar) = &t[1] {
			if let ethabi::Token::FixedArray(arr) = &ar[0] {
				if let ethabi::Token::Uint(u) = &arr[0] {
					b00 = u.to_string();
				}
				if let ethabi::Token::Uint(u) = &arr[1] {
					b01 = u.to_string();
				}
			}

			if let ethabi::Token::FixedArray(ar) = &t[1] {
				if let ethabi::Token::FixedArray(arr) = &ar[1] {
					if let ethabi::Token::Uint(u) = &arr[0] {
						b10 = u.to_string();
					}
					if let ethabi::Token::Uint(u) = &arr[1] {
						b11 = u.to_string();
					}
				}
			}
		}

		if let ethabi::Token::FixedArray(ar) = &t[2] {
			if let ethabi::Token::Uint(u) = &ar[0] {
				c0 = u.to_string();
			}
			if let ethabi::Token::Uint(u) = &ar[1] {
				c1 = u.to_string();
			}
		}
	}

	return (
		vec![a0, a1],
		vec![vec![b00, b01], vec![b10, b11]],
		vec![c0, c1],
	);
}

#[cfg(test)]
mod tests {
	use frame_support::assert_ok;
	use hex_literal::hex;
	use sp_core::H256;
	use sp_io::hashing::sha2_256;

	use crate::verifier::{decode_proof, Verifier};

	#[test]
	fn test_zk_step_with_serde() {
		let vk = r#"{"vk_json":{
 "protocol": "groth16",
 "curve": "bn128",
 "nPublic": 1,
 "vk_alpha_1": [
  "20491192805390485299153009773594534940189261866228447918068658471970481763042",
  "9383485363053290200918347156157836566562967994039712273449902621266178545958",
  "1"
 ],
 "vk_beta_2": [
  [
   "4252822878758300859123897981450591353533073413197771768651442665752259397132",
   "21847035105528745403288232691147584728191162732299865338377159692350059136679"
  ],
  [
   "6375614351688725206403948262868962793625744043794305715222011528459656738731",
   "10505242626370262277552901082094356697409835680220590971873171140371331206856"
  ],
  [
   "1",
   "0"
  ]
 ],
 "vk_gamma_2": [
  [
   "11559732032986387107991004021392285783925812861821192530917403151452391805634",
   "4082367875863433681332203403145435568316851327593401208105741076214120093531"
  ],
  [
   "10857046999023057135944570762232829481370756359578518086990519993285655852781",
   "8495653923123431417604973247489272438418190587263600148770280649306958101930"
  ],
  [
   "1",
   "0"
  ]
 ],
 "vk_delta_2": [
  [
   "11559732032986387107991004021392285783925812861821192530917403151452391805634",
   "4082367875863433681332203403145435568316851327593401208105741076214120093531"
  ],
  [
   "10857046999023057135944570762232829481370756359578518086990519993285655852781",
   "8495653923123431417604973247489272438418190587263600148770280649306958101930"
  ],
  [
   "1",
   "0"
  ]
 ],
 "vk_alphabeta_12": [],
 "IC": [
  [
   "202333273032481017331373350816007583026713320195536354260471885571526195724",
   "8246242704115088390751476790768744984402990892657920674334938931948100192840",
   "1"
  ],
  [
   "12901454334783146822957332552289769626984444933652541503990843020723194328882",
   "12436078488518552293095332739673622487901350475115357313978341690183990059269",
   "1"
  ],
  [
  "12828056956769114977702246128118682473179646035440405756936949778100648490262",
  "7351319165217643779735289066901404053730163225836026220896225559268517203790",
  "1"
  ]
 ]
}}"#;

		let v = Verifier::from_json_u8_slice(vk.as_bytes()).unwrap();

		assert_eq!("bn128", v.vk_json.curve);
		assert_eq!("groth16", v.vk_json.protocol);

		// TODO assert all points
		let inp = hex!(
			"0ab2afdc05c8b6ae1f2ab20874fb4159e25d5c1d4faa41aee232d6ab331332df0000000000747ffe"
		);
		let out = hex!("e4566e0cf4edb171a3eedd59f9943bbcd0b1f6b648f1a6e26d5264b668ab41ec51e76629b32b943497207e7b7ccff8fbc12e9e6d758cc7eed972422c4cad02b90000000000747fa001fd");
		let inp_hash = H256(sha2_256(inp.as_slice()));
		let out_hash = H256(sha2_256(out.as_slice()));

		// TODO remove println
		println!("{}", inp_hash);
		println!("{}", out_hash);
		let proof = hex!("10344e73ac87a69d9faece1099a4c1194d922a6d5eb24408508d426d699589f625a2326df0cd67d15a41c8d69da4faa766cac27f0d204e0eb2043d7083be90c12b011afc70277abd7e81aad605d6819cff24a36c8c1e47c4ce8e0eb6ce7c80910f4e62def9ed381d5d5ddd32b6e6255d0a6741ee3a446a0f6b5dd413be44bc261183316749a6483965dc3aac1c512d0f6fc485ef457cfd98343c92262447b4b62960747f517610464e2a896cb8153e9dcda32fa7c21176a5f6fe0707520a4d212e262c0c32f250ca50eba6c4495eb888f220551ff96cdf75a7310a5b11c223e4178daea92bacbe1ec3796115e41f47a33e1658e8620c33bf134f055fd645c4ce");

		let result = v.verify(inp_hash, out_hash, proof.to_vec());

		assert_ok!(result.clone());
		assert_eq!(true, result.unwrap());
	}

	#[test]
	fn test_decode_proof() {
		let proof = hex!("1332c772a8f9a02f304b5472d3b6b75f1a494bd9b137fc663fd5b9b475992bc829ba08f7cfa745e340938e356b139224d0288b9511a5cec83235f969f61a94ed16a14579fa0adcc3bf8da36209f64547fd5ff4e1c7e8b5b151335b5b4a471de3115f83b696517ac68ae7620f7d3840e44aff4781c0a4d265a2905ef9bcaa04432a660197790e60d1135946ae0603ef69a5ecb45b6f8046167f902dc6d8a35cf716bce116484dfa4fcd5d8f4c2fda26d68754b56e68f1a877d95dc171accc34d71285068693fe3d8d28e66342c31292ceee5c6d87fcb8ad8c132363565f2aeff905726b2d35def5c9636dd5ec402d8d6f6c9a7be7977e7e5727da327ea5b079ad");

		let decoded: (Vec<String>, Vec<Vec<String>>, Vec<String>) = decode_proof(proof.to_vec());

		assert_eq!(
			"8683663015073067038244847214283351810649000192281314413199884219842452597704",
			decoded.0[0]
		);
		assert_eq!(
			"18873522240908759015197166908776808810045074443031598381394130502027574940909",
			decoded.0[1]
		);
		assert_eq!(
			"10235824555245197129038838261179705064387070473723531210466639418098968894947",
			decoded.1[0][0]
		);
		assert_eq!(
			"7858077948381560609212308446029826533408997041182829878371843519718814778435",
			decoded.1[0][1]
		);
		assert_eq!(
			"19177369026551579179894492468331397687405155911290633487631565284771023248631",
			decoded.1[1][0]
		);
		assert_eq!(
			"10284603410671614550643238877116026784009997646397200180055169244522533893335",
			decoded.1[1][1]
		);
		assert_eq!(
			"8376666972810749572085581968561346381911579868801081275529626269155085447161",
			decoded.2[0]
		);
		assert_eq!(
			"2463724514031046292864306191243943409912346551164607808423034641717054699949",
			decoded.2[1]
		);
	}
}
