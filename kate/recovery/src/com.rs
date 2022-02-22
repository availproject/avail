use dusk_plonk::{fft::EvaluationDomain, prelude::BlsScalar};

// This module is taken from https://gist.github.com/itzmeanjan/4acf9338d9233e79cfbee5d311e7a0b4
// which I wrote few months back when exploring polynomial based erasure coding technique !

pub fn reconstruct_poly(
	// domain I'm working with
	// all (i)ffts to be performed on it
	eval_domain: EvaluationDomain,
	// subset of available data
	subset: Vec<Option<BlsScalar>>,
) -> Result<Vec<BlsScalar>, String> {
	let mut missing_indices = Vec::new();
	for i in 0..subset.len() {
		if let None = subset[i] {
			missing_indices.push(i as u64);
		}
	}
	let (mut zero_poly, zero_eval) =
		zero_poly_fn(eval_domain, &missing_indices[..], subset.len() as u64);
	for i in 0..subset.len() {
		if let None = subset[i] {
			if !(zero_eval[i] == BlsScalar::zero()) {
				return Err("bad zero poly evaluation !".to_owned());
			}
		}
	}
	let mut poly_evals_with_zero: Vec<BlsScalar> = Vec::new();
	for i in 0..subset.len() {
		if let Some(v) = subset[i] {
			poly_evals_with_zero.push(v * zero_eval[i]);
		} else {
			poly_evals_with_zero.push(BlsScalar::zero());
		}
	}
	let mut poly_with_zero = eval_domain.ifft(&poly_evals_with_zero[..]);
	shift_poly(&mut poly_with_zero[..]);
	shift_poly(&mut zero_poly[..]);
	let mut eval_shifted_poly_with_zero = eval_domain.fft(&poly_with_zero[..]);
	let eval_shifted_zero_poly = eval_domain.fft(&zero_poly[..]);
	for i in 0..eval_shifted_poly_with_zero.len() {
		eval_shifted_poly_with_zero[i] *= eval_shifted_zero_poly[i].invert().unwrap();
	}

	let mut shifted_reconstructed_poly = eval_domain.ifft(&eval_shifted_poly_with_zero[..]);
	unshift_poly(&mut shifted_reconstructed_poly[..]);

	let reconstructed_data = eval_domain.fft(&shifted_reconstructed_poly[..]);
	Ok(reconstructed_data)
}

fn expand_root_of_unity(eval_domain: EvaluationDomain) -> Vec<BlsScalar> {
	let root_of_unity = eval_domain.group_gen;
	let mut roots: Vec<BlsScalar> = Vec::new();
	roots.push(BlsScalar::one());
	roots.push(root_of_unity);
	let mut i = 1;
	while roots[i] != BlsScalar::one() {
		roots.push(roots[i] * root_of_unity);
		i += 1;
	}
	return roots;
}

fn zero_poly_fn(
	eval_domain: EvaluationDomain,
	missing_indices: &[u64],
	length: u64,
) -> (Vec<BlsScalar>, Vec<BlsScalar>) {
	let expanded_r_o_u = expand_root_of_unity(eval_domain);
	let domain_stride = eval_domain.size() as u64 / length;
	let mut zero_poly: Vec<BlsScalar> = Vec::with_capacity(length as usize);
	let mut sub: BlsScalar;
	for i in 0..missing_indices.len() {
		let v = missing_indices[i as usize];
		sub = BlsScalar::zero() - expanded_r_o_u[(v * domain_stride) as usize];
		zero_poly.push(sub);
		if i > 0 {
			zero_poly[i] = zero_poly[i] + zero_poly[i - 1];
			for j in (1..i).rev() {
				zero_poly[j] = zero_poly[j] * sub;
				zero_poly[j] = zero_poly[j] + zero_poly[j - 1];
			}
			zero_poly[0] = zero_poly[0] * sub
		}
	}
	zero_poly.push(BlsScalar::one());
	for _ in zero_poly.len()..zero_poly.capacity() {
		zero_poly.push(BlsScalar::zero());
	}
	let zero_eval = eval_domain.fft(&zero_poly[..]);
	(zero_poly, zero_eval)
}

// in-place shifting
fn shift_poly(poly: &mut [BlsScalar]) {
	// primitive root of unity
	let shift_factor = BlsScalar::from(5);
	let mut factor_power = BlsScalar::one();
	// hoping it won't panic, though it should be handled properly
	//
	// this is actually 1/ shift_factor --- multiplicative inverse
	let inv_factor = shift_factor.invert().unwrap();

	for i in 0..poly.len() {
		poly[i] *= factor_power;
		factor_power *= inv_factor;
	}
}

// in-place unshifting
fn unshift_poly(poly: &mut [BlsScalar]) {
	// primitive root of unity
	let shift_factor = BlsScalar::from(5);
	let mut factor_power = BlsScalar::one();

	for i in 0..poly.len() {
		poly[i] *= factor_power;
		factor_power *= shift_factor;
	}
}

#[cfg(test)]
mod tests {
	use std::time::{SystemTime, UNIX_EPOCH};

	use dusk_bytes::Serializable;
	use rand::{rngs::StdRng, Rng, SeedableRng};

	use super::*;

	#[test]
	fn data_reconstruction_success() {
		let domain_size = 1usize << 4;
		let eval_domain = EvaluationDomain::new(domain_size * 2).unwrap();

		// some dummy source data I care about
		let mut src: Vec<BlsScalar> = Vec::with_capacity(domain_size * 2);
		for i in 0..domain_size {
			src.push(BlsScalar::from(1 << (i + 1)));
		}
		// fill extended portion of vector with zeros
		for _ in domain_size..(2 * domain_size) {
			src.push(BlsScalar::zero());
		}

		// erasure code it
		let coded_src = eval_domain.fft(&src);
		// choose random subset of it ( >= 50% )
		let (coded_src_subset, _) = random_subset(&coded_src);
		// reconstruct 100% erasure coded values from random coded subset
		let coded_recovered = reconstruct_poly(eval_domain, coded_src_subset).unwrap();

		for i in 0..(2 * domain_size) {
			assert_eq!(coded_src[i], coded_recovered[i]);
		}

		let dst = eval_domain.ifft(&coded_recovered);

		for i in 0..(2 * domain_size) {
			assert_eq!(src[i].to_bytes(), dst[i].to_bytes());
		}
	}

	#[test]
	fn data_reconstruction_failure_0() {
		let domain_size = 1usize << 4;
		let eval_domain = EvaluationDomain::new(domain_size * 2).unwrap();

		let mut src: Vec<BlsScalar> = Vec::with_capacity(domain_size * 2);
		for i in 0..domain_size {
			src.push(BlsScalar::from(1 << (i + 1)));
		}
		for _ in domain_size..(2 * domain_size) {
			src.push(BlsScalar::zero());
		}

		let coded_src = eval_domain.fft(&src);
		let (mut coded_src_subset, available) = random_subset(&coded_src);
		// intentionally drop a few coded elements such that
		// < 50% is available
		drop_few(&mut coded_src_subset, available);
		// attempt to reconstruct 100% coded data from <50 % coded data
		// I've available
		let coded_recovered = reconstruct_poly(eval_domain, coded_src_subset).unwrap();

		let mut mismatch_count = 0;
		for i in 0..(2 * domain_size) {
			if coded_src[i] != coded_recovered[i] {
				mismatch_count += 1;
			}
		}

		assert!(mismatch_count > 0);
	}

	// Context behind following two test cases, where one failure condition
	// along with one possible solution, is demonstrated
	//
	// Need for writing these test cases originates in a conversation
	// with Prabal<https://github.com/prabal-banerjee> where we were discussing
	// how to ensure input byte chunks to dusk-plonk's `BlsScalar::from_bytes()`
	// is always lesser than prime field modulus ( 255 bits wide ), because
	// we'll get data bytes from arbitrary sources which will be concatenated into
	// single large byte array & finally (multiple) field elements to be produced by chunking contiguous bytes,
	// splitting bytearray into smaller chunks, each of size 32 bytes.
	//
	// Now imagine we got a 32-bytes wide chunk with content like [0xff; 32] --- all 256 -bits are set
	//
	// When that's attempted to be converted into field element it should be wrapped
	// around and original value will be lost
	//
	// We want to specify a way for `how a large byte string can be splitted into field elements
	// such that no values are required to be wrapped around due to modular division i.e. all
	// values must be lesser than 255-bit prime before they are attempted to be converted to BLS scalar ?`
	//
	// One natural way to think about solving this problem is grouping large byte array into 254-bits
	// chunks, then we should not encounter that problem as value is always lesser than
	// prime number which is 255 -bits. But that requires indexing within a byte i.e. not at byte boundary.
	//
	// **Solution** So we decided to chunk contiguous 31 bytes from large input byte array and
	// append zero byte(s) to each chunk for making 32 -bytes wide before inputting
	// 256 -bit integer to `BlsScalar::from_bytes( ... )` function
	//
	// Note, this means, for each field element of 256 -bits, we've 6 -bits free to use
	// and at this moment that's just set to zeros !
	// Other 2 -bits of MSB will always be 0 for avoiding modular division related issue.
	//
	// Is there a way to make use of 6 -bits of most significant byte, in every field element ?
	//
	// Test `data_reconstruction_failure_1` shows how chunking with 32 contiguous bytes
	// results into error where data is lost due to modular division
	//
	// While `data_reconstruction_failure_2` shows how chunking 31 bytes together
	// makes it work smoothly without any data loss encountered !
	#[test]
	#[should_panic]
	fn data_reconstruction_failure_1() {
		const GROUP_TOGETHER: usize = 32; // bytes

		let input = [0xffu8; (32 << 4) + 1];

		let domain_size = ((input.len() as f64) / GROUP_TOGETHER as f64).ceil() as usize;
		let eval_domain = EvaluationDomain::new(domain_size * 2).unwrap();

		let mut input_wide: Vec<[u8; 32]> = Vec::with_capacity(domain_size);

		for chunk in input.chunks(GROUP_TOGETHER) {
			let widened: [u8; 32] = {
				let mut v = vec![];
				v.extend_from_slice(&chunk.to_vec()[..]);
				// pad last chunk with required -many zeros
				for _ in 0..(GROUP_TOGETHER - chunk.len()) {
					v.push(0u8);
				} // v is 32 -bytes
				v.try_into().unwrap()
			};

			input_wide.push(widened);
		}

		let mut src: Vec<BlsScalar> = Vec::with_capacity(domain_size * 2);
		for i in 0..domain_size {
			src.push(BlsScalar::from_bytes(&input_wide[i]).unwrap());
		}
		for _ in 0..domain_size {
			src.push(BlsScalar::zero());
		}

		// erasure code it
		let coded_src = eval_domain.fft(&src);
		// choose random subset of it ( >= 50% )
		let (coded_src_subset, _) = random_subset(&coded_src);
		// reconstruct 100% erasure coded values from random coded subset
		let coded_recovered = reconstruct_poly(eval_domain, coded_src_subset).unwrap();

		for i in 0..(2 * domain_size) {
			assert_eq!(coded_src[i], coded_recovered[i]);
		}

		let dst = eval_domain.ifft(&coded_recovered);

		for i in 0..domain_size {
			let chunk_0 = if (i + 1) * GROUP_TOGETHER >= input.len() {
				&input[i * GROUP_TOGETHER..]
			} else {
				&input[i * GROUP_TOGETHER..(i + 1) * GROUP_TOGETHER]
			};
			let chunk_1 = &dst[i].to_bytes()[..chunk_0.len()];

			assert_eq!(chunk_0, chunk_1, "{}", format!("at i = {}", i));
		}
		for i in domain_size..(2 * domain_size) {
			assert_eq!(
				[0u8; GROUP_TOGETHER],
				dst[i].to_bytes(),
				"{}",
				format!("at i = {}", i)
			);
		}
	}

	#[test]
	fn data_reconstruction_failure_2() {
		const GROUP_TOGETHER: usize = 31; // bytes

		let input = [0xffu8; 32 << 4];

		let domain_size = ((input.len() as f64) / GROUP_TOGETHER as f64).ceil() as usize;
		let eval_domain = EvaluationDomain::new(domain_size * 2).unwrap();

		let mut input_wide: Vec<[u8; 32]> = Vec::with_capacity(domain_size);

		for chunk in input.chunks(GROUP_TOGETHER) {
			let widened: [u8; 32] = {
				let mut v = vec![];
				v.extend_from_slice(&chunk.to_vec()[..]);
				// pad last chunk with required -many zeros
				for _ in 0..(GROUP_TOGETHER - chunk.len()) {
					v.push(0u8);
				} // v is 31 -bytes
				v.push(0u8); // v is now 32 -bytes
				v.try_into().unwrap()
			};

			input_wide.push(widened);
		}

		let mut src: Vec<BlsScalar> = Vec::with_capacity(domain_size * 2);
		for i in 0..domain_size {
			src.push(BlsScalar::from_bytes(&input_wide[i]).unwrap());
		}
		for _ in 0..domain_size {
			src.push(BlsScalar::zero());
		}

		// erasure code it
		let coded_src = eval_domain.fft(&src);
		// choose random subset of it ( >= 50% )
		let (coded_src_subset, _) = random_subset(&coded_src);
		// reconstruct 100% erasure coded values from random coded subset
		let coded_recovered = reconstruct_poly(eval_domain, coded_src_subset).unwrap();

		for i in 0..(2 * domain_size) {
			assert_eq!(coded_src[i], coded_recovered[i]);
		}

		let dst = eval_domain.ifft(&coded_recovered);

		for i in 0..domain_size {
			let chunk_0 = if (i + 1) * GROUP_TOGETHER >= input.len() {
				&input[i * GROUP_TOGETHER..]
			} else {
				&input[i * GROUP_TOGETHER..(i + 1) * GROUP_TOGETHER]
			};
			let chunk_1 = &dst[i].to_bytes()[..chunk_0.len()];

			assert_eq!(chunk_0, chunk_1, "{}", format!("at i = {}", i));
		}
		for i in domain_size..(2 * domain_size) {
			assert_eq!(
				[0u8; GROUP_TOGETHER + 1],
				dst[i].to_bytes(),
				"{}",
				format!("at i = {}", i)
			);
		}
	}

	fn drop_few(data: &mut [Option<BlsScalar>], mut available: usize) {
		assert!(available <= data.len());

		let mut idx = 0;
		while available >= data.len() / 2 {
			if let Some(_) = data[idx] {
				data[idx] = None;
				available -= 1;
			}
			idx += 1;
		}
	}

	// select a random subset of coded data to be used for
	// reconstruction purpose
	//
	// @note this is just a helper function for writing test case
	fn random_subset(data: &[BlsScalar]) -> (Vec<Option<BlsScalar>>, usize) {
		let mut rng = StdRng::seed_from_u64(
			SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.unwrap()
				.as_secs(),
		);
		let mut subset: Vec<Option<BlsScalar>> = Vec::with_capacity(data.len());
		let mut available = 0;
		for i in 0..data.len() {
			if rng.gen::<u8>() % 2 == 0 {
				subset.push(Some(data[i]));
				available += 1;
			} else {
				subset.push(None);
			}
		}

		// already we've >=50% data available
		// so just return & attempt to reconstruct back
		if available >= data.len() / 2 {
			(subset, available)
		} else {
			for i in 0..data.len() {
				if let None = subset[i] {
					// enough data added, >=50% needs
					// to be present
					if available >= data.len() / 2 {
						break;
					}

					subset[i] = Some(data[i]);
					available += 1;
				}
			}
			(subset, available)
		}
	}
}
