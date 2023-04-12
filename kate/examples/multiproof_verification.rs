use da_types::{AppExtrinsic, AppId};
use hex_literal::hex;
use kate::{Seed, Serializable};
use poly_multiproof::{
	ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress},
	merlin::Transcript,
	traits::PolyMultiProofNoPrecomp,
};
use rand::thread_rng;

fn main() {
	let target_dims = kate::grid::Dimensions::new_unchecked(64, 16);
	let pp = kate::testnet::public_params(256.into());
	let pmp = poly_multiproof::m1_blst::M1NoPrecomp::new(256, 256, &mut thread_rng());
	let (proof, evals, commitments, dims) = {
		let exts = vec![
			AppExtrinsic {
				app_id: AppId(0),
				data: hex!("CAFEBABE").to_vec(),
			},
			AppExtrinsic {
				app_id: AppId(1),
				data: hex!("DEADBEEF").to_vec(),
			},
			AppExtrinsic {
				app_id: AppId(2),
				data: hex!("12345678").to_vec(),
			},
		];
		let seed = Seed::default();
		let grid = kate::gridgen::EvaluationGrid::from_extrinsics(exts, 4, 256, 256, seed)
			.unwrap()
			.extend_columns(2)
			.unwrap();

		// Setup, serializing as bytes
		let polys = grid.make_polynomial_grid().unwrap();

		let commitments = polys
			.commitments(&pp.commit_key())
			.unwrap()
			.iter()
			.flat_map(|c| c.0.to_bytes())
			.collect::<Vec<_>>();

		let multiproof = polys
			.multiproof(
				&pmp,
				&kate::com::Cell {
					row: 0.into(),
					col: 0.into(),
				},
				&grid,
				&target_dims,
			)
			.unwrap();

		for r in &multiproof.evals {
			for e in r {
				assert!(e.serialized_size(Compress::Yes) == 32)
			}
		}

		let mut proof_bytes = [0u8; 48];
		// TODO: better proof ser
		multiproof
			.proof
			.0
			.serialize_compressed(&mut proof_bytes[..])
			.unwrap();

		// TODO: better evals ser
		let evals_bytes = multiproof
			.evals
			.iter()
			.flat_map(|row| {
				row.iter().flat_map(|e| {
					let mut out = [0u8; 32];
					e.serialize_uncompressed(&mut out[..]).unwrap();
					out
				})
			})
			.collect::<Vec<_>>();
		(proof_bytes, evals_bytes, commitments, grid.dims)
	};

	let mp_block = kate::gridgen::multiproof_block(0, 0, &dims, &target_dims).unwrap();
	let commits = commitments
		.chunks_exact(48)
		.skip(mp_block.start_y)
		.take(mp_block.end_y - mp_block.start_y)
		.map(|c| {
			let mut out = [0u8; 48];
			out.copy_from_slice(c);
			kate::pmp::Commitment(
				kate::pmp::m1_blst::G1Affine::deserialize_compressed(&out[..]).unwrap(),
			)
		})
		.collect::<Vec<_>>();

	type Fr = kate::pmp::m1_blst::Fr;
	use kate::pmp::ark_poly::EvaluationDomain;
	let points = kate::pmp::ark_poly::GeneralEvaluationDomain::<Fr>::new(dims.width())
		.unwrap()
		.elements()
		.skip(mp_block.start_x)
		.take(mp_block.end_x - mp_block.start_x)
		.collect::<Vec<_>>();

	let block_commits = &commits[mp_block.start_x..mp_block.end_x];
	let evals_flat = evals
		.chunks_exact(32)
		.map(|e| {
			let mut out = [0u8; 32];
			out.copy_from_slice(e);
			kate::pmp::m1_blst::Fr::deserialize_compressed(&out[..]).unwrap()
		})
		.collect::<Vec<_>>();
	let evals_grid = evals_flat
		.chunks_exact(mp_block.end_x - mp_block.start_x)
		.collect::<Vec<_>>();

	let proof_point = kate::pmp::m1_blst::G1Affine::deserialize_compressed(&proof[..]).unwrap();
	let proof = kate::pmp::m1_blst::Proof(proof_point);

	pmp.verify(
		&mut Transcript::new(b"avail-mp"),
		block_commits,
		&points,
		&evals_grid,
		&proof,
	)
	.unwrap();
}
