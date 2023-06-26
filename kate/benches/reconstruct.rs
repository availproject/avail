use criterion::{black_box, criterion_group, criterion_main, Criterion};
use da_primitives::{BlockLengthColumns, BlockLengthRows};
use da_types::{AppExtrinsic, AppId};
use dusk_plonk::prelude::BlsScalar;
use hex_literal::hex;
use kate::{
	com::{Cell, *},
	metrics::IgnoreMetrics,
	BlockDimensions, Seed, Serializable,
};
use kate_recovery::{
	com::reconstruct_extrinsics,
	commitments,
	data::{self, DataCell},
	index::*,
	matrix::Position,
	proof, testnet,
};
use rand::{prelude::IteratorRandom, Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use sp_arithmetic::{traits::SaturatedConversion, Percent};

#[rustfmt::skip]
fn make_xts() -> Vec<AppExtrinsic> {
	vec![
		AppExtrinsic { 
			app_id: AppId(0),
			data: hex!("1470af08b0ab8ff9d79a3b6402c83fd92e4e26f6d9c60ca360144939362192441ae63d4c94a36f5e151d719366b9c214a9ff240ff6cc9385d8fb95acf51a8e6f07255e08cf3e283e14dca7aebe1e5afdb0502a780404702bd93711305375883738300450de7a8d8f98ff961bd887db94a83d06ccf622201f3c73a9c317c214ff9bd76eeff7ce6bb6fa4ce552800b57207a3738ed78540ff5a089f76b26ca4f4e9fc4968e75fc3eb9d81e58ec680e429741840abc372e226ad785795a5bdea186ada577ff610fee6233822513e98ce5a0e462deda92cbc2e576c3723c0e86c4a838fa55db0252c2c78730d3a4d7746e2fc54bde37a0181a161e09ec093391f3a79a5fd4f950e286aafb8168bb74d59742bdb74feab83d6d353d65a30e0c16793ee614ee1fcfc20d63137681315245508e20982133ebfa3bbc58679cd2e3cc6ac055474f826d8824d1d9e37fef906f3d145f4864f79112b5689341df0b410b6013974a5fa8b3ccac836c6ffe060915c5fa74ee0d4b15cd43581fb7d1ce2dcbdab4850fd4145598a031b8994bcceb12b6b7a8ca3f5c7aac39b8d42a590a5e82631872a4f637d4106a3ab293aa84ff076d4859eb164ed00f727d").to_vec()
		},
		AppExtrinsic {
			app_id: AppId(14183),
			data: hex!("d48abcb0d4c6acef7119109d66187fba860fbd07283895235c5a60a987bdcf5125c61f5dc93d82daa5701c53e4a87461662d09e3919e6ae29eba8f907a223f5243111c4627f29bc8eaa0556f819d09aa8456a94b0d494d4bc3472e0b34339334d890c38746d398110b4bb2b830f533c544a96bfcacd40d9c02abccd2832e3159ec3dd105d6aa7cf0a15a43af1f1aea9a88ec9a2fc2f6f371811837486202ead04d77ad5c75c2b93f3cff29c990d90b151d0c8c506cf2d9ff9d0aae0e564f6c07b46a58b14d75b54365c03e4266f0c96c83b92969f109705d1000fc634c934648d90c9c73ca7fce164b4855817b242ca7448b50179f9d167be3ee27cdb6570cdefcb2b8cc6521d094ba74fa2838b1df6f48e044d013e88aec33de9a7f66bd5f2302ad298581f163ab843ec3a0252c1a6ed40ec5c15f28e63ee2c36821c61ab60e433ee7c8faa662a1a225f29dbe99af00d664dec4ad77a174b6f8ff7176f21f6953e0034ac66aa0a46bf87f4809a68c4d02fc63e9ade900119fee98dda988e6dd72a1fcac98e43bdfd1cbff48781918aa7ef7695e949d96c0fcaa65959ea3c24e3f66410c6f6e0d932aef9ed823d05bb048f93773e10e807e5e862dfb56fef4e39640e0b6b13e8ab0345b60ef0dbe57d289896fd002adc8a43021a629023116094020b2ce6aec1c1305b3124a42750a5be72577756e2a5306d0d8e8f0b1226216fc11a49cede06352bcc58bf854970a2ea3da95fa334c6e34bd6ba181ff2aaa918263210e998a7101f6cd111c827d0ed220328594688a9b011bbf753885e2ceb0f6974e1371c4655b10f38d81bfa8d092c13f8c5e4387d7ec09ccebf10bd2ebb82de30ff67fb1c493fdec2878fe0cb3cb024dea02c3caa8a82a8be85a5ac904e96c22d0c32bf7a5ea3c703d168ed8049cb7d4cf8f612fa7576814f4e09db516e97e2c82e8eb6976bdde44dc11a351a72cb8e9f6a7f14ca56c5b192a6c889c5d02137ad5a1b83ef1e8ffb1917d98624253b3fcd38afeaa7cfb0904a2ced2dcb51af8fcc9e6733368e18e55bc7f264e1e915b1d10772013e508b72a9def320913a8e6787523c69db034bd99d70fbdf8bfc4d1c137d741e9e0e4109546586601681815c0ea679942c0dfdeff3ef4ee285313d826076b9e84b2e17a30f274a7cf97665ab56c7da309d01191d5fb52c45c036025fd75f56353f6337bea19888d8b63e47de8a09a71793af2d370dbff6010ac6d26fbbb1f92fdad47446b60eb4d1cc04ebd20fe545f4c61f3bb1f0ba73458392259ed4d2e1a18acdf158002a72ff91f6fc690f5f7d1b9ce6bf1c91065169d6486db016ec10a8cd916089a79e7fd44ae6530144df23ed367e2c599dff0ef14215bf7deaf63911d453fa0a3d84d3f7319eed77bfa9e2df0e2a658077569a66bbd71b7ff856bcd9bc089e2ccbee7d7b12ce48c496b18673125ca32465af19796ee0edf53dcff48911fcd09af6647a081cb3126118b6974b905067e8a4985dec6f289f64abd6feaa975449df12119675981e603f9897876449faa6fe81a6d96771f5c35664017816fc0c953ff1aa4087385284ccbe9e7ae068da5ac015af82687dcbdc96a10e929a2ab40dff231b7cbbde59b5004e0150cbb0c6c7fe1291835d7e876dba7a31b0109ba76f8a9df4fd4af339e762931f942a29f593f61e12d407545f6a404f6e6505a348616f97fb0baf033b450c5bea2321b22eca83a8750efb98d1336864dec21ae9ba2b1b8b5319e631f9dfbb455dae7aa4e0e25a2cba68e14db323033eb16ed94614e85fe8829b35cb6f5422a7374a4cf6e21ff1445f28f89179fa75947b106dba902cd744d326cd26b71fbfac0f46f4cf26425f67ab7fa5cff0d9dd19293f54ae9a7115b2bf955424a93ab0f8067e55df600ad9cb49eaa1b9f40447f9896b75eae0e7d44ce6631ead3a39c81b17c24782ad015ac2306a75c80691a0f881787eb95029f3f66fded61ecbaa7b13692e3e920d3331afbafc8097028feea068e7badda27e45cb8422ed045474a83e08d8a9869a1f87bc45fc202fa2fc8cd0343e5d02fc1880c7ac717240002b171a23589dde0ae710c721b61fd418a3ef349b6c0b9c9abb9d134bc7b621b227f3e2021f9c95d1ad0bf6ce4e442f98b55115a73c934911373a4fc299979a014b61b5d88c8346829fff83cb3c1ec89e67d43be2e2aa80e0a85f7d434c47eb2c85af02084c2d2c90b972807322f45513fe7df4436494b161fe15a7e9bf8393ec9fa8cc0235a64bd29c429b3fc871cfc13415df3b9fe05748eb7e205c0d7e5").to_vec(),
		},
		AppExtrinsic { 
			app_id: AppId(117134629),
			data: hex!("9e6e075b63217f5b6cd1de4f824c49ce9123ea0a1307ab6a4a0c7296e9affd35073784c001ab15cb826c17ae606ade3b937beb3ef44187b1f73e1451c1a52d36523df6aedf40b835132ef770e54bf91511990da102f2fa54219b8633770865fe92d8c4b041354444f039541741445cb90509251f6fa377aa74bb82748da8ca2a6e1dc86164c9a01f0c3fb93d095dd999a0d66fe07dcd5eb9065ff0d227bcb66ff841d0bf0a9ce80647581c458e3bf15916aa1345c1ec99b140deaf1c6cd29f0c57a956dd230b8958e0d1c94e92568619167bbc06e0548298a24889421dada1765a1642bd84753e8da9155d311234e674bdad61cfd7a64b165374aa4687e7dae1a1f95a3cb2697dd0d8363436253b7acd55e853210519f81bb19ac55ef1a6d0a8a128").to_vec(),
		},
		AppExtrinsic { 
			app_id: AppId(117134630),
			data: hex!("488945bd0005807499f3f3fc0b395b607ed35d2d4bc6ac8b9ab0ff5cb36f58fd2237144c312dbc658e11fec7990febe8ffe4373e33bcfb5189a690b11e473aeb6d57787ef6ea0909e7988d993e583f589b31e8da63fe014db6d1fcadc4a6e99b15d21cfa5ff00cd93d89224b7bfccf7cb44f9b727bf7994b849300a8a4254feab27c9fc3918e4206febe64daa2b5f715fc6763d4fc1ece9be8424ab1db4bb843d097f66568101e586e47b220cf61a0ec635e0cb4490abaa4fefbdad6588eb3e670d1037845257f98971e014f9079ce507f660bf27d25704908dfa2520a92dd06feca0d8737a7c774ceaa1ba9887ff398da09b21bd78fa8dc835a5731d4a4914eddef16209d14e319a809306b62180fbf8d6fa5662e4f1ab09a1efe358a9a88a52393b825120648af932ba1dcd2d47a0ccadb0ba96e10d04d02afaeeee7c332560ebe54f7697ffb9a405398cc489dcf4812771731e9a39b375b37a35bdec4180fd0647f0daaad1327a7f1f6053125a8d64956123fa22d1cc2528f595465b924ed14142e97e0a92c34fbcf76a199c2fe84efb4cc7de2f0024ed5b29b0b81e2786652a8fceac787b23054466151600b5ecc47abd930b80cc78f6abf2811f6f93d33600fe3bf22bf8087d3d39df459170a7e7c26e3f143531208b2702002937eee2b5acdc2bda278c23455b14b060b01a8b9aa57e8ba499f0a38d429872e7701bd8b1f8161aaeec6f46d5f9c996fd83053f9dd787a4586107204d5d0bcb8abfe043bdb5c01d3b8fd667e8d8fc6e8ea7a2e8eb2fa9879b9d2ddaad1bf8550c61f7ac853eb8e9b708eb8eff4cc7adfab147dc355d27ddb0ce3cf78106c871855e1f9cbb340ed0652e691ef657f5a19f2f3f710f668121dea55727497633773b5e0abd7acf97d313139be57ff556a728933b1fccf3203071ac494686343530ac8b5a31951a9ba86048870cbaf626417b8278e8382dabf680da5d8d9de5dfaf6ed54b321c794dca67c10bb0e7e4e9e4b5a2e2edb9f5b5ed5188e7694488b39da2e8a0266569dc08e6e06a68e698085326b6b456d89993e72bddcf522c1f70a1d986c54bbb8328893e56a7fb58ec162dc5b31fefb94c417ce6bda86125b6b0ef4d97fef83bfa38b901f8b7bdce5d0b27c841dba04a99b6b0d88a9d5ae387f193bf4a40e2b4f301f7e63195a1102ec9f5779c9cdac0bcdc0c04c318a848bb018903e225df771fe92bca9b592681f584b9cb484eb2bf6cfdcc616cd08e16ff306b67f09b18279f3ee8fb30bddff62251452482b25980a08c6fa1d8d3e0118204269323e61f43e513f14c6a46a638a1159abe7b1acacbfae6d057e7eebcb03562aba7460a66fa1c547e857b31faea87a6e028fec4d3f05550e5e7af60fbb6e793ecd9bcf85b36a6244995ec33a85d627d9fdf47f185d4ad6fc90af245c6ba5b74bd69e28d29cb311da691308e7a89888dd54b8f4e760c8b809ef1a821507ba26dbdc411af54fddd9d8dd36062fa7f39b4b8293188813d7d93f74a7eade8b8132ab6a393fe4a92ee3eeb1526a0dab793ba41e6e92d9").to_vec(),
		}
	]
}

fn sample_cells_from_matrix(
	matrix: &[BlsScalar],
	dimensions: &BlockDimensions,
	columns: Option<&[u16]>,
) -> Vec<DataCell> {
	fn random_indexes(length: usize, seed: Seed) -> Vec<u16> {
		// choose random len/2 (unique) indexes
		let mut idx = (0..length).collect::<Vec<_>>();
		let mut chosen_idx = Vec::<u16>::new();
		let mut rng = ChaChaRng::from_seed(seed);

		for _ in 0..length / 2 {
			let i = rng.gen_range(0..idx.len());
			let v = idx.remove(i);
			chosen_idx.push(v as u16);
		}
		chosen_idx
	}

	const RNG_SEED: Seed = [42u8; 32];
	matrix
		.chunks_exact(dimensions.rows.as_usize().saturating_mul(2))
		.enumerate()
		.map(|(col, e)| (col as u16, e))
		.flat_map(|(col, e)| {
			random_indexes(e.len(), RNG_SEED)
				.into_iter()
				.map(|row| DataCell {
					position: Position {
						row: row as u32,
						col,
					},
					data: e[row as usize].to_bytes(),
				})
				.filter(|cell| {
					columns.is_none() || columns.unwrap_or(&[]).contains(&cell.position.col)
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

fn app_data_index_try_from_layout(
	layout: Vec<(AppId, u32)>,
) -> Result<AppDataIndex, AppDataIndexError> {
	let mut index = Vec::new();
	// transactions are ordered by application id
	// skip transactions with 0 application id - it's not a data txs
	let mut size = 0u32;
	let mut prev_app_id = AppId(0u32);

	for (app_id, data_len) in layout {
		if app_id.0 != 0 && prev_app_id != app_id {
			index.push((app_id.0, size));
		}

		size = size
			.checked_add(data_len)
			.ok_or(AppDataIndexError::SizeOverflow)?;
		if prev_app_id > app_id {
			return Err(AppDataIndexError::UnsortedLayout);
		}
		prev_app_id = app_id;
	}

	Ok(AppDataIndex { size, index })
}

fn random_cells(
	max_cols: BlockLengthColumns,
	max_rows: BlockLengthRows,
	percents: Percent,
) -> Vec<Cell> {
	let max_cols = max_cols.into();
	let max_rows = max_rows.into();

	let rng = &mut ChaChaRng::from_seed([0u8; 32]);
	let amount: usize = percents
		.mul_ceil::<u32>(max_cols * max_rows)
		.saturated_into();

	(0..max_cols)
		.flat_map(move |col| {
			(0..max_rows).map(move |row| Cell::new(BlockLengthRows(row), BlockLengthColumns(col)))
		})
		.choose_multiple(rng, amount)
}

fn bench_reconstruct(c: &mut Criterion) {
	c.bench_function("reconstruct", |b| b.iter(|| reconstruct()));
}

fn reconstruct() {
	let xts = make_xts();

	let metrics = IgnoreMetrics {};
	let (layout, commitments, dims, matrix) = par_build_commitments(
		BlockLengthRows(64),
		BlockLengthColumns(16),
		32,
		xts.as_slice(),
		Seed::default(),
		&metrics,
	)
	.unwrap();

	let columns = sample_cells_from_matrix(&matrix, &dims, None);
	let extended_dims = dims.try_into().unwrap();
	let index = app_data_index_try_from_layout(layout).unwrap();
	let reconstructed = reconstruct_extrinsics(&index, &extended_dims, columns).unwrap();
	for (result, xt) in reconstructed.iter().zip(xts.into_iter()) {
		assert_eq!(result.0, *xt.app_id);
		assert_eq!(result.1[0].as_slice(), &xt.data);
	}

	let public_params = testnet::public_params(dims.cols.as_usize());
	for cell in random_cells(dims.cols, dims.rows, Percent::one()) {
		let row = cell.row.as_usize();

		let proof = build_proof(&public_params, dims, &matrix, &[cell], &metrics).unwrap();
		assert_eq!(proof.len(), 80);

		let col: u16 = cell
			.col
			.0
			.try_into()
			.expect("`random_cells` function generates a valid `u16` for columns");
		let position = Position {
			row: cell.row.0,
			col,
		};
		let cell = data::Cell {
			position,
			content: proof.try_into().unwrap(),
		};

		let extended_dims = dims.try_into().unwrap();
		let commitment = commitments::from_slice(&commitments).unwrap()[row];
		let verification = proof::verify(&public_params, &extended_dims, &commitment, &cell);
		assert!(verification.is_ok());
		assert!(verification.unwrap());
	}
}

criterion_group! { benches, bench_reconstruct }
criterion_main!(benches);
