// // MIT License
//
// // Copyright (c) 2022 Bright Inventions
//
// // Permission is hereby granted, free of charge, to any
// // person obtaining a copy of this software and associated
// // documentation files (the "Software"), to deal in the
// // Software without restriction, including without
// // limitation the rights to use, copy, modify, merge,
// // publish, distribute, sublicense, and/or sell copies of
// // the Software, and to permit persons to whom the Software
// // is furnished to do so, subject to the following
// // conditions:
//
// // The above copyright notice and this permission notice
// // shall be included in all copies or substantial portions
// // of the Software.
//
// // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// // ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// // TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// // PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// // SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// // CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// // OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// // IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// // DEALINGS IN THE SOFTWARE.
//
// use serde::{Deserialize, Deserializer};
// use sp_std::vec::Vec;
// use uint::construct_uint;
//
// construct_uint! {
// 	pub struct U256(6);
// }
//
// type Number = [u8; 48];
//
// type G1 = [Number; 3];
// type G2 = [[Number; 2]; 3];
//
// // #[serde(rename = "IC")]
// // pub ic: Vec<Vec<String>>,
// //
// // #[serde(rename = "nPublic")]
// // pub inputs_count: u32,
// // pub vk_alpha_1: Vec<String>,
// // pub vk_beta_2: Vec<Vec<String>>,
// // pub vk_gamma_2: Vec<Vec<String>>,
// // pub vk_delta_2: Vec<Vec<String>>,
// // pub vk_alphabeta_12: Vec<Vec<Vec<String>>>,
// // pub curve: String,
// // pub protocol: String,
//
// /// Struct representing snarkjs generated verification key
// #[derive(Deserialize)]
// pub struct VKey {
// 	#[serde(deserialize_with = "str_to_u8_vec_deserializer")]
// 	pub protocol: Vec<u8>,
// 	#[serde(deserialize_with = "str_to_u8_vec_deserializer")]
// 	pub curve: Vec<u8>,
// 	#[serde(alias = "nPublic")]
// 	pub public_inputs_len: u8,
// 	#[serde(alias = "vk_alpha_1")]
// 	#[serde(deserialize_with = "g1_deserializer")]
// 	pub alpha: G1,
// 	#[serde(alias = "vk_beta_2")]
// 	#[serde(deserialize_with = "g2_deserializer")]
// 	pub beta: G2,
// 	#[serde(alias = "vk_gamma_2")]
// 	#[serde(deserialize_with = "g2_deserializer")]
// 	pub gamma: G2,
// 	#[serde(alias = "vk_delta_2")]
// 	#[serde(deserialize_with = "g2_deserializer")]
// 	pub delta: G2,
// 	#[serde(alias = "IC")]
// 	#[serde(deserialize_with = "vec_g1_deserializer")]
// 	pub ic: Vec<G1>,
// }
//
// #[derive(Debug)]
// pub enum VKeyDeserializationError {
// 	SerdeError,
// }
//
// impl VKey {
// 	/// Creates `VKey` from json representation
// 	pub fn from_json_u8_slice(slice: &[u8]) -> Result<Self, VKeyDeserializationError> {
// 		serde_json::from_slice(slice).map_err(|_| VKeyDeserializationError::SerdeError)
// 	}
// }
//
// /// Struct representing snarkjs generated proof
// #[derive(Deserialize)]
// pub struct Proof {
// 	#[serde(deserialize_with = "str_to_u8_vec_deserializer")]
// 	pub protocol: Vec<u8>,
// 	#[serde(deserialize_with = "str_to_u8_vec_deserializer")]
// 	pub curve: Vec<u8>,
// 	#[serde(alias = "pi_a")]
// 	#[serde(deserialize_with = "g1_deserializer")]
// 	pub a: G1,
// 	#[serde(alias = "pi_b")]
// 	#[serde(deserialize_with = "g2_deserializer")]
// 	pub b: G2,
// 	#[serde(alias = "pi_c")]
// 	#[serde(deserialize_with = "g1_deserializer")]
// 	pub c: G1,
// }
//
// #[derive(Debug)]
// pub enum ProofDeserializationError {
// 	SerdeError,
// }
//
// impl Proof {
// 	/// Creates `Proof` from json representation
// 	pub fn from_json_u8_slice(slice: &[u8]) -> Result<Self, ProofDeserializationError> {
// 		serde_json::from_slice(slice).map_err(|_| ProofDeserializationError::SerdeError)
// 	}
// }
// /// Turns G1 point represented by numbers in decimal format into G1 point represented by numbers in
// /// binary format
// pub fn g1_deserializer<'de, D>(de: D) -> Result<[Number; 3], D::Error>
// where
// 	D: Deserializer<'de>,
// {
// 	let mut dec_numbers: [Number; 3] = [[0; 48]; 3];
// 	let s: [&str; 3] = serde::Deserialize::deserialize(de)?;
// 	for i in 0..3 {
// 		U256::from_dec_str(s[i])
// 			.unwrap()
// 			.to_big_endian(dec_numbers[i].as_mut_slice());
// 	}
// 	Ok(dec_numbers)
// }
//
// /// Turns array of G1 points represented by numbers in decimal format into vector of G1 points
// /// represented by numbers in binary format
// pub fn vec_g1_deserializer<'de, D>(de: D) -> Result<Vec<[Number; 3]>, D::Error>
// where
// 	D: Deserializer<'de>,
// {
// 	let dec_numbers: Vec<[&str; 3]> = serde::Deserialize::deserialize(de)?;
// 	Ok(dec_numbers
// 		.iter()
// 		.map(|ic| {
// 			let mut arr: [Number; 3] = [[0; 48]; 3];
// 			for i in 0..3 {
// 				U256::from_dec_str(ic[i])
// 					.unwrap()
// 					.to_big_endian(arr[i].as_mut_slice());
// 			}
// 			arr
// 		})
// 		.collect())
// }
//
// /// Turns G2 point represented by numbers in decimal format into G2 point represented by numbers in
// /// binary format
// pub fn g2_deserializer<'de, D>(de: D) -> Result<[[Number; 2]; 3], D::Error>
// where
// 	D: Deserializer<'de>,
// {
// 	let mut g2_numbers: [[Number; 2]; 3] = [[[0; 48]; 2]; 3];
// 	let dec_numbers: [[&str; 2]; 3] = serde::Deserialize::deserialize(de)?;
// 	for i in 0..3 {
// 		for j in 0..2 {
// 			U256::from_dec_str(dec_numbers[i][j])
// 				.unwrap()
// 				.to_big_endian(g2_numbers[i][j].as_mut_slice());
// 		}
// 	}
// 	Ok(g2_numbers)
// }
//
// /// Turns `str` into `Vec<u8>`
// pub fn str_to_u8_vec_deserializer<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
// where
// 	D: Deserializer<'de>,
// {
// 	let s: &str = serde::Deserialize::deserialize(de)?;
// 	Ok(s.as_bytes().into())
// }
//
// #[derive(Debug)]
// pub enum PublicInputsDeserializationError {
// 	SerdeError,
// }
//
// /// Creates vector of `u64` representing public inputs
// ///
// /// # Arguments
// /// * `inputs` - A byte array slice containing array of integers in json array form
// pub fn deserialize_public_inputs(
// 	inputs: &[u8],
// ) -> Result<Vec<u64>, PublicInputsDeserializationError> {
// 	let inputs: Vec<&str> = serde_json::from_slice(inputs).unwrap();
// 	let mut parsed_inputs: Vec<u64> = Vec::with_capacity(inputs.len());
// 	for input in inputs {
// 		match input.parse::<u64>() {
// 			Ok(n) => parsed_inputs.push(n),
// 			Err(_) => return Err(PublicInputsDeserializationError::SerdeError),
// 		}
// 	}
// 	Ok(parsed_inputs)
// }
//
// #[cfg(test)]
// mod tests {
// 	use crate::deserialization::{deserialize_public_inputs, Number, Proof, VKey, U256};
//
// 	#[test]
// 	fn test_vk_deserialization() {
// 		let vk = r#"{
//  "protocol": "groth16",
//  "curve": "bls12381",
//  "nPublic": 1,
//  "vk_alpha_1": [
//   "2635983656263320256511463995836413167331869092392943593306076905516259749312747842295447349507189592731785901862558",
//   "743892996456702519498029594549937288641619275055957975879157306988929970626325326222697609972550552691064908651931",
//   "1"
//  ],
//  "vk_beta_2": [
//   [
//    "1296094501238138520689116246487755613076576267512760150298482409401507546730337296489670510433482825207790998397346",
//    "3467549840163329914429787393326495235851806074050417925094845001935796859739058829480949031354270816778136382040361"
//   ],
//   [
//    "3403410200913851046378881164751590587066009874691619938225021193334979700147466129997648606538377099567064346931273",
//    "3804847074485539411700684267722735363688167108429634491643293100788171321105199556340902873511607444652008144844173"
//   ],
//   [
//    "1",
//    "0"
//   ]
//  ],
//  "vk_gamma_2": [
//   [
//    "352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160",
//    "3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758"
//   ],
//   [
//    "1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905",
//    "927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582"
//   ],
//   [
//    "1",
//    "0"
//   ]
//  ],
//  "vk_delta_2": [
//   [
//    "3284950120787447527021651154232749836311526699432807747366843303661566424019671125328694916562846460813145647040459",
//    "3218306808275776807419693666072599084905639169987324420818366627509865827220976650759812930713956208246000627242485"
//   ],
//   [
//    "3945290144137392347873751586031392152201459997902585432454016489727689337013866944382877542451368688652560743518350",
//    "3505020872425170466568261366418107787485649574216477007429328593907934456720034754142706045279478244784882964969099"
//   ],
//   [
//    "1",
//    "0"
//   ]
//  ],
//  "vk_alphabeta_12": [
//   [
//    [
//     "2875682627859788046787727046323207700818211438271057184016590533515641699292997693457599620936708894947724425715231",
//     "1238832727101571020174962081840437018121939792461931445079462232794726384752259447129206595688503509959353794284793"
//    ],
//    [
//     "1142295393527745936520465586775444768688364741373237930118445421796520414741916849824256960449892474465014692624756",
//     "2180077006016464788050801868062734927906767334913187269719534809313436039282935753136702491423916116028147695108113"
//    ],
//    [
//     "581912189975592585217934845255593126879157415518933223520266217690258707840190591176174259119561150786976644927862",
//     "1496521185256234033198775390415811847166244093737149241712223242576005202124661827966889067451826629794876020037891"
//    ]
//   ],
//   [
//    [
//     "968778761326544533347894440852946317832878172436078056438728764792716948106777133186592741979864246862480026990714",
//     "3286237875677076419439678035167386721716851772116127087476697302808027553397980022598381369816552966804476744614726"
//    ],
//    [
//     "703046133019192877150497098682775062870944581080811653558417167836034365682308629278579084636589495681129838804552",
//     "3120651492951743750811126470515331662411558962596191689455151216422711804034698152168980665082907679235009776566592"
//    ],
//    [
//     "3093035865177537484265129293484086930964325066660842965056946750881983192007730606218463861804151316907199193750598",
//     "2217088332657331378025998358211322741524769834682072728928845130805944349335376146743275401044544953792401446016391"
//    ]
//   ]
//  ],
//  "IC": [
//   [
//    "3759794041598018594287463133849401670165044879836734797942436987012929463856866218164906521458646350224910548839839",
//    "3238512100593065266229132824040292706800754984648723917955334599968665051423411534393542324672325614522917210582797",
//    "1"
//   ],
//   [
//    "3305881491744710205856868316456114914540772066725994230747514104922282269209779243587827394909802115252764372519712",
//    "2462443929524735084767395208674598757462820081953985438437610428598624587728712969052746628125821805697605346885091",
//    "1"
//   ]
//  ]
// }"#;
// 		let v_key: VKey = VKey::from_json_u8_slice(vk.as_bytes()).unwrap();
//
// 		assert_eq!(v_key.alpha[0], from_dec_string("2635983656263320256511463995836413167331869092392943593306076905516259749312747842295447349507189592731785901862558"));
// 		assert_eq!(v_key.alpha[1], from_dec_string("743892996456702519498029594549937288641619275055957975879157306988929970626325326222697609972550552691064908651931"));
// 		assert_eq!(v_key.alpha[2], from_dec_string("1"));
//
// 		assert_eq!(v_key.beta[0][0], from_dec_string("1296094501238138520689116246487755613076576267512760150298482409401507546730337296489670510433482825207790998397346"));
// 		assert_eq!(v_key.beta[0][1], from_dec_string("3467549840163329914429787393326495235851806074050417925094845001935796859739058829480949031354270816778136382040361"));
// 		assert_eq!(v_key.beta[1][0], from_dec_string("3403410200913851046378881164751590587066009874691619938225021193334979700147466129997648606538377099567064346931273"));
// 		assert_eq!(v_key.beta[1][1], from_dec_string("3804847074485539411700684267722735363688167108429634491643293100788171321105199556340902873511607444652008144844173"));
// 		assert_eq!(v_key.beta[2][0], from_dec_string("1"));
// 		assert_eq!(v_key.beta[2][1], from_dec_string("0"));
//
// 		assert_eq!(v_key.gamma[0][0], from_dec_string("352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160"));
// 		assert_eq!(v_key.gamma[0][1], from_dec_string("3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758"));
// 		assert_eq!(v_key.gamma[1][0], from_dec_string("1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905"));
// 		assert_eq!(v_key.gamma[1][1], from_dec_string("927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582"));
// 		assert_eq!(v_key.gamma[2][0], from_dec_string("1"));
// 		assert_eq!(v_key.gamma[2][1], from_dec_string("0"));
//
// 		assert_eq!(v_key.delta[0][0], from_dec_string("3284950120787447527021651154232749836311526699432807747366843303661566424019671125328694916562846460813145647040459"));
// 		assert_eq!(v_key.delta[0][1], from_dec_string("3218306808275776807419693666072599084905639169987324420818366627509865827220976650759812930713956208246000627242485"));
// 		assert_eq!(v_key.delta[1][0], from_dec_string("3945290144137392347873751586031392152201459997902585432454016489727689337013866944382877542451368688652560743518350"));
// 		assert_eq!(v_key.delta[1][1], from_dec_string("3505020872425170466568261366418107787485649574216477007429328593907934456720034754142706045279478244784882964969099"));
// 		assert_eq!(v_key.delta[2][0], from_dec_string("1"));
// 		assert_eq!(v_key.delta[2][1], from_dec_string("0"));
//
// 		assert_eq!(v_key.ic[0][0], from_dec_string("3759794041598018594287463133849401670165044879836734797942436987012929463856866218164906521458646350224910548839839"));
// 		assert_eq!(v_key.ic[0][1], from_dec_string("3238512100593065266229132824040292706800754984648723917955334599968665051423411534393542324672325614522917210582797"));
// 		assert_eq!(v_key.ic[0][2], from_dec_string("1"));
//
// 		assert_eq!(v_key.ic[1][0], from_dec_string("3305881491744710205856868316456114914540772066725994230747514104922282269209779243587827394909802115252764372519712"));
// 		assert_eq!(v_key.ic[1][1], from_dec_string("2462443929524735084767395208674598757462820081953985438437610428598624587728712969052746628125821805697605346885091"));
// 		assert_eq!(v_key.ic[1][2], from_dec_string("1"));
//
// 		assert_eq!(v_key.curve, Vec::<u8>::from("bls12381".as_bytes()));
// 		assert_eq!(v_key.protocol, Vec::<u8>::from("groth16".as_bytes()));
// 		assert_eq!(v_key.public_inputs_len, 1);
// 	}
//
// 	#[test]
// 	fn test_proof_deserialization() {
// 		let proof = r#"{
//  "pi_a": [
//   "2820173869801000183955769496344276101575675010174203082588560105436284422640780128231242184109173767085197647834267",
//   "1152541093585973172499551859168528642628429504007613830168996825879806250289422935864437193085184388469171892221011",
//   "1"
//  ],
//  "pi_b": [
//   [
//    "54413090665354594353317256815335793052197307111690011609872716599840507808706991989403605000342095250665180513594",
//    "3343285764332210309442703216841128605678475246673133285314301861643378387001264758819444434632415207857557469906035"
//   ],
//   [
//    "262180403851765105493218367619507205740764669171746348153545090105487398261554724750259283942935411519021270362742",
//    "3777303780170739988308854254585940898119682705621212814969008224084499326863117961704608873229314936725151172212883"
//   ],
//   [
//    "1",
//    "0"
//   ]
//  ],
//  "pi_c": [
//   "3006923877346016048391409264528383002939756547318806158402407618139299715778986391175418348881376388499383266389442",
//   "1307513151230758506579817970515482216448699470263630520204374492458260823057506418477833081567163581258564509876945",
//   "1"
//  ],
//  "protocol": "groth16",
//  "curve": "bls12381"
// }"#;
//
// 		let proof: Proof = Proof::from_json_u8_slice(proof.as_bytes()).unwrap();
//
// 		assert_eq!(proof.a[0], from_dec_string("2820173869801000183955769496344276101575675010174203082588560105436284422640780128231242184109173767085197647834267"));
// 		assert_eq!(proof.a[1], from_dec_string("1152541093585973172499551859168528642628429504007613830168996825879806250289422935864437193085184388469171892221011"));
// 		assert_eq!(proof.a[2], from_dec_string("1"));
//
// 		assert_eq!(proof.b[0][0], from_dec_string("54413090665354594353317256815335793052197307111690011609872716599840507808706991989403605000342095250665180513594"));
// 		assert_eq!(proof.b[0][1], from_dec_string("3343285764332210309442703216841128605678475246673133285314301861643378387001264758819444434632415207857557469906035"));
// 		assert_eq!(proof.b[1][0], from_dec_string("262180403851765105493218367619507205740764669171746348153545090105487398261554724750259283942935411519021270362742"));
// 		assert_eq!(proof.b[1][1], from_dec_string("3777303780170739988308854254585940898119682705621212814969008224084499326863117961704608873229314936725151172212883"));
// 		assert_eq!(proof.b[2][0], from_dec_string("1"));
// 		assert_eq!(proof.b[2][1], from_dec_string("0"));
//
// 		assert_eq!(proof.c[0], from_dec_string("3006923877346016048391409264528383002939756547318806158402407618139299715778986391175418348881376388499383266389442"));
// 		assert_eq!(proof.c[1], from_dec_string("1307513151230758506579817970515482216448699470263630520204374492458260823057506418477833081567163581258564509876945"));
// 		assert_eq!(proof.c[2], from_dec_string("1"));
//
// 		assert_eq!(proof.curve, Vec::<u8>::from("bls12381".as_bytes()));
// 		assert_eq!(proof.protocol, Vec::<u8>::from("groth16".as_bytes()));
// 	}
//
// 	#[test]
// 	fn public_inputs_deserialization() {
// 		let public_inputs_json = r#"[
//  "33"
// ]"#;
// 		let public_inputs =
// 			deserialize_public_inputs(public_inputs_json.as_bytes().into()).unwrap();
// 		assert_eq!(public_inputs.len(), 1);
// 		assert_eq!(public_inputs[0], 33);
// 	}
//
// 	fn from_dec_string(dec_str: &str) -> Number {
// 		let mut number: Number = [0; 48];
// 		U256::from_dec_str(dec_str)
// 			.unwrap()
// 			.to_big_endian(number.as_mut_slice());
// 		number
// 	}
// }
