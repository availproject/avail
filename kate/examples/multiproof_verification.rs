use da_types::{AppExtrinsic, AppId};
use hex_literal::hex;
use kate::{pmp::{merlin::Transcript, traits::PolyMultiProofNoPrecomp}, Seed};
use poly_multiproof::traits::AsBytes;
use rand::thread_rng;

fn main() {
	let target_dims = kate::grid::Dimensions::new_unchecked(16, 64);
	let pp = kate::testnet::multiproof_params(256, 256);
	let pmp = poly_multiproof::m1_blst::M1NoPrecomp::new(256, 256, &mut thread_rng());
	let points = kate::gridgen::domain_points(256).unwrap();
	let (proof, evals, commitments, dims) = {
		let exts = vec![
			AppExtrinsic {
				app_id: AppId(0),
				data: hex!("CAFEBABE00000000000000000000000000000000000000").to_vec(),
			},
			AppExtrinsic {
				app_id: AppId(1),
				data: hex!("DEADBEEF1111111111111111111111111111111111").to_vec(),
			},
			AppExtrinsic {
				app_id: AppId(2),
				data: hex!("1234567899999999999999999999999999999999").to_vec(),
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
			.commitments(&pp)
			.unwrap()
			.iter()
			.flat_map(|c| c.to_bytes().unwrap())
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

		let proof_bytes = multiproof.proof.to_bytes().unwrap();
		let evals_bytes = multiproof
			.evals
			.iter()
			.flat_map(|row| row.iter().flat_map(|e| e.to_bytes().unwrap()))
			.collect::<Vec<_>>();
		(proof_bytes, evals_bytes, commitments, grid.dims)
	};

	let mp_block = kate::gridgen::multiproof_block(0, 0, &dims, &target_dims).unwrap();
	let commits = commitments
		.chunks_exact(48)
		.skip(mp_block.start_y)
		.take(mp_block.end_y - mp_block.start_y)
		.map(|c| kate::pmp::Commitment::from_bytes(c.try_into().unwrap()))
		.collect::<Result<Vec<_>, _>>()
		.unwrap();

	let block_commits = &commits[mp_block.start_x..mp_block.end_x];
	let evals_flat = evals
		.chunks_exact(32)
		.map(|e| kate::gridgen::ArkScalar::from_bytes(e.try_into().unwrap()))
		.collect::<Result<Vec<_>, _>>()
		.unwrap();
	let evals_grid = evals_flat
		.chunks_exact(mp_block.end_x - mp_block.start_x)
		.collect::<Vec<_>>();

	let proof = kate::pmp::m1_blst::Proof::from_bytes(&proof).unwrap();

	pmp.verify(
		&mut Transcript::new(b"avail-mp"),
		block_commits,
		&points[mp_block.start_x..mp_block.end_x],
		&evals_grid,
		&proof,
	)
	.unwrap();
}
