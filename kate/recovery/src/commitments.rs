use std::{
	array::TryFromSliceError,
	convert::{TryFrom, TryInto},
	num::TryFromIntError,
};

use dusk_bytes::Serializable;
use dusk_plonk::{
	fft::{EvaluationDomain, Evaluations},
	prelude::{BlsScalar, PublicParameters},
};
use thiserror::Error;

use crate::{
	com,
	config::{self, COMMITMENT_SIZE},
	index, matrix,
};

#[derive(Error, Debug)]
pub enum DataError {
	#[error("Scalar slice error: {0}")]
	SliceError(TryFromSliceError),
	#[error("Scalar data is not valid")]
	ScalarDataError,
	#[error("Invalid scalar data length")]
	BadScalarDataLen,
	#[error("Scalar data contains invalid character")]
	BadScalarData,
	#[error("Bad data len")]
	BadLen,
	#[error("Plonk error: {0}")]
	PlonkError(dusk_plonk::error::Error),
	#[error("Bad commitments data")]
	BadCommitmentsData,
	#[error("Bad rows data")]
	BadRowsData,
}

#[derive(Error, Debug)]
pub enum Error {
	#[error("Invalid data: {0}")]
	InvalidData(DataError),
}

impl From<TryFromSliceError> for Error {
	fn from(e: TryFromSliceError) -> Self {
		Self::InvalidData(DataError::SliceError(e))
	}
}

impl From<TryFromIntError> for Error {
	fn from(_: TryFromIntError) -> Self {
		Self::InvalidData(DataError::BadCommitmentsData)
	}
}

impl From<dusk_bytes::Error> for Error {
	fn from(e: dusk_bytes::Error) -> Self {
		match e {
			dusk_bytes::Error::InvalidData => Self::InvalidData(DataError::ScalarDataError),
			dusk_bytes::Error::BadLength { .. } => Self::InvalidData(DataError::BadScalarDataLen),
			dusk_bytes::Error::InvalidChar { .. } => Self::InvalidData(DataError::BadScalarData),
		}
	}
}

impl From<dusk_plonk::error::Error> for Error {
	fn from(e: dusk_plonk::error::Error) -> Self {
		Self::InvalidData(DataError::PlonkError(e))
	}
}

fn try_into_scalar(chunk: &[u8]) -> Result<BlsScalar, Error> {
	let sized_chunk = <[u8; config::CHUNK_SIZE]>::try_from(chunk)?;
	BlsScalar::from_bytes(&sized_chunk).map_err(From::from)
}

fn try_into_scalars(data: &[u8]) -> Result<Vec<BlsScalar>, Error> {
	let chunks = data.chunks_exact(config::CHUNK_SIZE);
	if !chunks.remainder().is_empty() {
		return Err(Error::InvalidData(DataError::BadLen));
	}
	chunks
		.map(try_into_scalar)
		.collect::<Result<Vec<BlsScalar>, Error>>()
}

/// Verifies given commitments and row commitments equality.
/// Commitments are verified only for app specific data rows.
/// Function returns pair of verified and missing data rows, or an error.
/// Invalid rows are treated as missing.
///
/// # Arguments
///
/// * `public_params` - Public parameters
/// * `commitments` - Commitments represented as byte array (as in header)
/// * `rows` - Array of optional rows
/// * `index` - Application data index
/// * `dimensions` - Extended matrix dimensions
/// * `app_id` - Application ID
pub fn verify_equality(
	public_params: &PublicParameters,
	commitments: &[[u8; COMMITMENT_SIZE]],
	rows: &[Option<Vec<u8>>],
	index: &index::AppDataIndex,
	dimensions: &matrix::Dimensions,
	app_id: u32,
) -> Result<(Vec<u32>, Vec<u32>), Error> {
	if commitments.len() != dimensions.extended_rows().try_into()? {
		return Err(Error::InvalidData(DataError::BadCommitmentsData));
	}

	let mut app_rows = com::app_specific_rows(index, dimensions, app_id);

	if rows.len() != dimensions.extended_rows().try_into()? {
		return Ok((vec![], app_rows));
	}

	let (prover_key, _) = public_params.trim(dimensions.cols() as usize)?;
	let domain = EvaluationDomain::new(dimensions.cols() as usize)?;

	// This is a single-threaded implementation.
	// At some point we should benchmark and decide
	// if we need parallel commitments verification.
	let verified = commitments
		.iter()
		.zip(rows.iter())
		.zip(0u32..)
		.filter(|(.., index)| app_rows.contains(index))
		.filter_map(|((&commitment, row), index)| {
			try_into_scalars(row.as_ref()?)
				.map(|scalars| Evaluations::from_vec_and_domain(scalars, domain).interpolate())
				.and_then(|polynomial| prover_key.commit(&polynomial).map_err(From::from))
				.map(|result| (result.to_bytes() == commitment).then_some(index))
				.transpose()
		})
		.collect::<Result<Vec<u32>, Error>>()?;

	app_rows.retain(|row| !verified.contains(row));

	Ok((verified, app_rows))
}

/// Creates vector of exact size commitments, from commitments slice
pub fn from_slice(source: &[u8]) -> Result<Vec<[u8; COMMITMENT_SIZE]>, TryFromSliceError> {
	source
		.chunks(COMMITMENT_SIZE)
		.map(TryInto::try_into)
		.collect::<Result<_, _>>()
}

#[cfg(test)]
mod tests {
	use dusk_plonk::prelude::PublicParameters;
	use once_cell::sync::Lazy;
	use rand::SeedableRng;
	use rand_chacha::ChaChaRng;

	use crate::{
		commitments,
		index::{self, AppDataIndex},
		matrix,
	};

	use super::verify_equality;

	static PUBLIC_PARAMETERS: Lazy<PublicParameters> =
		Lazy::new(|| PublicParameters::setup(256, &mut ChaChaRng::seed_from_u64(42)).unwrap());

	#[test]
	fn verify_equality_err() {
		assert!(super::verify_equality(
			&PUBLIC_PARAMETERS,
			&[],
			&[],
			&index::AppDataIndex::default(),
			&matrix::Dimensions::new(1, 1).unwrap(),
			0,
		)
		.is_err());
	}

	#[test]
	fn verify_equality_ok() {
		let commitments_bytes = hex::decode("ad98bf48c8c02ee04638a46cecc767678a41687cd2ae78ec13e2f0f078cf73ec200bd477f4a227f10a2a67acc88d5923805beb51d8b72b6aaae127870e971f457779819f7ae8dbfdcaa65b328a947f9949deae49a298456dee84f7df607f8cd98e7848b4d8e5aad76dc1a70c28e3b51d5f935cdfa95601f8fbc2293ed6fb03448eb4038cc718f6e67b61eb0d6b20f7b88738a3d6f31a808794d0cb5e76c476ac9aa964155b262789cd634aa545e54284950d18de93b2fde52144d03f5de7d32fa6b4182f3a4b150d033a207b366972788d1f3b52e24d1ea985044191d18fba3be181adae85caa96a7a85867460f4fc328de41695227f62d2a0ec8863ba61087258a39557b15c22c3c65c820f5f2cd5533967e08832a0e6bbb9d223c5553cbdd681ff913ffef22d5372bd422d567d3b110c17995f1ec0b335246a0037c91799a9b44fb368ad9259b0d0b959a9fbc38aacb618117e2c3261efd77da544bef8da8d3d6b4c2ead898c0c5fa0ae0d6acea9ee53262c15ef1d9e928e1fc0a5d85c4e78").unwrap();

		let commitments = commitments::from_slice(&commitments_bytes).unwrap();

		let row_0 = Some(hex::decode("04583c323032322d31312d3234205468752031303a31303e800000000000000004e1224120796f756e67206d61727469616c20617274697374206e616d65640020416c6578204b696464206c6561726e73206f6620612076696c6c61696e20006e616d6564204a616e6b656e207468652047726561742077686f2068617320006465666561746564204b696e67205468756e646572206f6620746865206369007479206f6620526164617869616e20616e642077686f20686173206b69646e0061707065642068697320736f6e2c205072696e63652045676c6520286f722000274967756c27292c20616e642045676c652773206669616e63656520507269006e63657373204c6f72612e20446973636f766572696e6720746861742068650020697320746865206c6f737420736f6e206f66204b696e67205468756e646500722c20416c65782073657473206f757420746f207265736375652074686520006b696e67646f6d2e204f6e206869732071756573742c206865206465666561007473204a616e6b656e27732068656e63686d656e20616e64207265747269650076657320766172696f7573206974656d73207768696368206c656164206869006d20746f77617264204a616e6b656e2077686f6d20686520646566656174730020616e642073656573207475726e656420746f2073746f6e652e20416c65780020726574726965766573207468652063726f776e2c20616e6420746865207000656f706c65206f6620526164617869616e2061726520726573746f7265642000756e64657220746865206e65776c792063726f776e6564204b696e67204567006c652e4120796f756e67206d61727469616c20617274697374206e616d65640020416c6578204b696464206c6561726e73206f6620612076696c6c61696e20006e616d6564204a616e6b656e207468652047726561742077686f2068617320006465666561746564204b696e67205468756e646572206f6620746865206369007479206f6620526164617869616e20616e642077686f20686173206b69646e0061707065642068697320736f6e2c205072696e63652045676c6520286f722000274967756c27292c20616e642045676c652773206669616e63656520507269006e63657373204c6f72612e20446973636f766572696e6720746861742068650020697320746865206c6f737420736f6e206f66204b696e67205468756e646500722c20416c65782073657473206f757420746f207265736375652074686520006b696e67646f6d2e204f6e206869732071756573742c206865206465666561007473204a616e6b656e27732068656e63686d656e20616e64207265747269650076657320766172696f7573206974656d73207768696368206c65616420686900").unwrap());

		let row_2 = Some(hex::decode("6d20746f77617264204a616e6b656e2077686f6d20686520646566656174730020616e642073656573207475726e656420746f2073746f6e652e20416c65780020726574726965766573207468652063726f776e2c20616e6420746865207000656f706c65206f6620526164617869616e2061726520726573746f7265642000756e64657220746865206e65776c792063726f776e6564204b696e67204567006c652e4120796f756e67206d61727469616c20617274697374206e616d65640020416c6578204b696464206c6561726e73206f6620612076696c6c61696e20006e616d6564204a616e6b656e207468652047726561742077686f2068617320006465666561746564204b696e67205468756e646572206f6620746865206369007479206f6620526164617869616e20616e642077686f20686173206b69646e0061707065642068697320736f6e2c205072696e63652045676c6520286f722000274967756c27292c20616e642045676c652773206669616e63656520507269006e63657373204c6f72612e20446973636f766572696e6720746861742068650020697320746865206c6f737420736f6e206f66204b696e67205468756e646500722c20416c65782073657473206f757420746f207265736375652074686520006b696e67646f6d2e204f6e206869732071756573742c206865206465666561007473204a616e6b656e27732068656e63686d656e20616e64207265747269650076657320766172696f7573206974656d73207768696368206c656164206869006d20746f77617264204a616e6b656e2077686f6d20686520646566656174730020616e642073656573207475726e656420746f2073746f6e652e20416c65780020726574726965766573207468652063726f776e2c20616e6420746865207000656f706c65206f6620526164617869616e2061726520726573746f7265642000756e64657220746865206e65776c792063726f776e6564204b696e67204567006c652e4120796f756e67206d61727469616c20617274697374206e616d65640020416c6578204b696464206c6561726e73206f6620612076696c6c61696e20006e616d6564204a616e6b656e207468652047726561742077686f2068617320006465666561746564204b696e67205468756e646572206f6620746865206369007479206f6620526164617869616e20616e642077686f20686173206b69646e0061707065642068697320736f6e2c205072696e63652045676c6520286f722000274967756c27292c20616e642045676c652773206669616e63656520507269006e63657373204c6f72612e20446973636f766572696e6720746861742068650020697320746865206c6f737420736f6e206f66204b696e67205468756e646500").unwrap());

		let row_4 = Some(hex::decode("722c20416c65782073657473206f757420746f207265736375652074686520006b696e67646f6d2e204f6e206869732071756573742c206865206465666561007473204a616e6b656e27732068656e63686d656e20616e64207265747269650076657320766172696f7573206974656d73207768696368206c656164206869006d20746f77617264204a616e6b656e2077686f6d20686520646566656174730020616e642073656573207475726e656420746f2073746f6e652e20416c65780020726574726965766573207468652063726f776e2c20616e6420746865207000656f706c65206f6620526164617869616e2061726520726573746f7265642000756e64657220746865206e65776c792063726f776e6564204b696e67204567006c652e800000000000000000000000000000000000000000000000000000000004fd01412072656d616b65206f66207468652067616d652c207469746c65640020416c6578204b69646420696e204d697261636c6520576f726c642044582c002077617320616e6e6f756e636564206f6e204a756e652031302c2032303230002c20616e642072656c6561736564206f6e204a756e652032322c2032303231002e2054686520800000000000000000000000000000000000000000000000000076a04053bda0a88bda5177b86a15c3b29f559873cb481232299cd5743151ac004b2d63ae198e7bb0a9011f28e473c95f4013d7d53ec5fbc3b42df8ed101f6d00e831e52bfb76e51cca8b4e9016838657edfae09cb9a71eb219025c4c87a67c004aaa86f20ac0aa792bc121ee42e2c326127061eda15599cb5db3db870bea5a00ecf353161c3cb528b0c5d98050c4570bfc942d8b19ed7b0cbba5725e03e5f000b7e30db36b6df82ac151f668f5f80a5e2a9cac7c64991dd6a6ce21c060175800edb9260d2a86c836efc05f17e5c59525e404c6a93d051651fe2e4eefae2813004925683890a942f63ce493f512f0b2cfb7c42a07ce9130cb6d059a388d886100536cb9c5b81a9a8dc46c2d64a7a5b1d93b2d8646805d8d2a122fccdb3bc7dc00975ab75fc865793536f66e64189050360f623dc88abb8300180cdd0a8f33d700d2159b3df296b46dd64bec57609a3f2fb4ad8b46e2fd4c9f25d44328dd50ce00514db7bbf50ef518c195a7053763d0a8dfdab6b946ee9f3954549319ac7dc600bac203232876b27b541433fb2f1438289799049b349f7a2c205d3a97f66ef4002800baa3cb78fb33130181775fb26a62630236bd8bc644a3656489d135ba1800b11846029a9183d434593cbbc1e03a4f8dba40cf6cfa07ba043c83f6a4888700364c233191a4b99aff1e9b8ab2aba54ecc61a6a8d2a50043e8948be1e76a43007d348990b99e55fee2a4bc79b29b27f2f9720e96840517dc8a0be65757110400").unwrap());

		let size = 79;
		let index = vec![(1, 1), (2, 74)];

		let result = verify_equality(
			&PUBLIC_PARAMETERS,
			&commitments,
			&[row_0.clone(), None, row_2, None, row_4, None, None, None],
			&AppDataIndex { size, index },
			&matrix::Dimensions::new(4, 32).unwrap(),
			1,
		);
		assert_eq!(result.unwrap(), (vec![0, 2, 4], vec![]));

		let size = 79;
		let index = vec![(1, 1), (2, 74)];

		let result = verify_equality(
			&PUBLIC_PARAMETERS,
			&commitments,
			&[row_0, None, None, None, None, None, None, None],
			&AppDataIndex { size, index },
			&matrix::Dimensions::new(4, 32).unwrap(),
			1,
		);
		assert_eq!(result.unwrap(), (vec![0], vec![2, 4]));

		let size = 79;
		let index = vec![(1, 1), (2, 74)];

		let result = verify_equality(
			&PUBLIC_PARAMETERS,
			&commitments,
			&[None, None, None, None, None, None, None, None],
			&AppDataIndex { size, index },
			&matrix::Dimensions::new(4, 32).unwrap(),
			1,
		);
		assert_eq!(result.unwrap(), (vec![], vec![0, 2, 4]));
	}
}
