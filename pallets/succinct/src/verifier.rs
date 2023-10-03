use ark_groth16::{prepare_verifying_key, verify_proof, Proof, VerifyingKey};

use ark_std::string::String;
use ark_std::vec::Vec;

use ark_bn254::{Bn254, Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_std::string::ToString;
use serde::{Deserialize, Serialize};

use ark_std::str::FromStr;
use ark_std::vec;

#[derive(Deserialize, Serialize)]
pub struct Verifier {
	vk_json: VerifyingKeyJson,
}

impl Verifier {
	pub fn new_step_verifier() -> Self {
		Self {
            vk_json: VerifyingKeyJson {
                ic: vec![
                    vec![
                        "14768330346746297840816367070658728893313212053739352195802618469166531204391".to_string(),
                        "226007277514949219964518589190903213280753732819328898150443666757283640566".to_string(),
                        "1".to_string(),
                    ],
                    vec![
                        "11579789275084599412171695990815953848893751967864880119324773293908098730772".to_string(),
                        "7016524000863123597202679959446996204295974709290664682467334394757983209848".to_string(),
                        "1".to_string(),
                    ],
                ],
                vk_alpha_1: vec!["20491192805390485299153009773594534940189261866228447918068658471970481763042".to_string(),
                                 "9383485363053290200918347156157836566562967994039712273449902621266178545958".to_string(),
                                 "1".to_string()],
                vk_beta_2: vec![
                    vec![
                        "6375614351688725206403948262868962793625744043794305715222011528459656738731".to_string(),
                        "4252822878758300859123897981450591353533073413197771768651442665752259397132".to_string(),
                    ],
                    vec![
                        "10505242626370262277552901082094356697409835680220590971873171140371331206856".to_string(),
                        "21847035105528745403288232691147584728191162732299865338377159692350059136679".to_string(),
                    ],
                    vec![
                        "1".to_string(),
                        "0".to_string(),
                    ],
                ],
                vk_gamma_2: vec![
                    vec![
                        "10857046999023057135944570762232829481370756359578518086990519993285655852781".to_string(),
                        "11559732032986387107991004021392285783925812861821192530917403151452391805634".to_string(),
                    ],
                    vec![
                        "8495653923123431417604973247489272438418190587263600148770280649306958101930".to_string(),
                        "4082367875863433681332203403145435568316851327593401208105741076214120093531".to_string(),
                    ],
                    vec![
                        "1".to_string(),
                        "0".to_string(),
                    ],
                ],
                vk_delta_2: vec![
                    vec![
                        "13909124302531010921185816266702828674819977847946098152869315744616458486564".to_string(),
                        "20132301864891590102651537900097603129841488311097169471951837821863335966377".to_string(),
                    ],
                    vec![
                        "9968363667543645393414941586581030294599633785037951467496223618072496422152".to_string(),
                        "19620890790369364323423864638476333921325558259845161848280036523505618212219".to_string(),
                    ],
                    vec![
                        "1".to_string(),
                        "0".to_string(),
                    ],
                ],
                vk_alphabeta_12: vec![
                    vec![
                        vec![
                            "2029413683389138792403550203267699914886160938906632433982220835551125967885".to_string(),
                            "21072700047562757817161031222997517981543347628379360635925549008442030252106".to_string(),
                        ],
                        vec![
                            "5940354580057074848093997050200682056184807770593307860589430076672439820312".to_string(),
                            "12156638873931618554171829126792193045421052652279363021382169897324752428276".to_string(),
                        ],
                        vec![
                            "7898200236362823042373859371574133993780991612861777490112507062703164551277".to_string(),
                            "7074218545237549455313236346927434013100842096812539264420499035217050630853".to_string(),
                        ],
                    ],
                    vec![
                        vec![
                            "7077479683546002997211712695946002074877511277312570035766170199895071832130".to_string(),
                            "10093483419865920389913245021038182291233451549023025229112148274109565435465".to_string(),
                        ],
                        vec![
                            "4595479056700221319381530156280926371456704509942304414423590385166031118820".to_string(),
                            "19831328484489333784475432780421641293929726139240675179672856274388269393268".to_string(),
                        ],
                        vec![
                            "11934129596455521040620786944827826205713621633706285934057045369193958244500".to_string(),
                            "8037395052364110730298837004334506829870972346962140206007064471173334027475".to_string(),
                        ],
                    ],
                ],
                inputs_count: 1,
                curve: "bn128".to_string(),
                protocol: "groth16".to_string(),
            },
        }
	}

	pub fn verify_proof(self, proof: Proof<Bn254>, inputs: &[Fr]) -> Result<bool, String> {
		let vk = self.vk_json.to_verifying_key();
		let pvk = prepare_verifying_key(&vk);

		return match verify_proof(&pvk, &proof, inputs) {
			Ok(r) => Ok(r),
			Err(e) => {
				// TOOD wrap error
				Err(e.to_string())
			},
		};
	}
}

#[derive(Deserialize, Serialize)]
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
