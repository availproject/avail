use ss58_registry::{from_known_address_format, Ss58AddressFormat, Ss58AddressFormatRegistry};

static DEFAULT_VERSION: core::sync::atomic::AtomicU16 = core::sync::atomic::AtomicU16::new(
	from_known_address_format(Ss58AddressFormatRegistry::SubstrateAccount),
);

/// Returns default SS58 format used by the current active process.
pub fn default_ss58_version() -> Ss58AddressFormat {
	DEFAULT_VERSION
		.load(core::sync::atomic::Ordering::Relaxed)
		.into()
}

/* /// Returns either the input address format or the default.
pub fn unwrap_or_default_ss58_version(network: Option<Ss58AddressFormat>) -> Ss58AddressFormat {
	network.unwrap_or_else(default_ss58_version)
} */

const PREFIX: &[u8] = b"SS58PRE";

fn ss58hash(data: &[u8]) -> Vec<u8> {
	use blake2::{Blake2b512, Digest};

	let mut ctx = Blake2b512::new();
	ctx.update(PREFIX);
	ctx.update(data);
	ctx.finalize().to_vec()
}

/// An error type for SS58 decoding.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PublicError {
	#[cfg_attr(feature = "std", error("Base 58 requirement is violated"))]
	BadBase58,
	#[cfg_attr(feature = "std", error("Length is bad"))]
	BadLength,
	#[cfg_attr(
		feature = "std",
		error(
			"Unknown SS58 address format `{}`. ` \
		`To support this address format, you need to call `set_default_ss58_version` at node start up.",
			_0
		)
	)]
	UnknownSs58AddressFormat(Ss58AddressFormat),
	#[cfg_attr(feature = "std", error("Invalid checksum"))]
	InvalidChecksum,
	#[cfg_attr(feature = "std", error("Invalid SS58 prefix byte."))]
	InvalidPrefix,
	#[cfg_attr(feature = "std", error("Invalid SS58 format."))]
	InvalidFormat,
	#[cfg_attr(feature = "std", error("Invalid derivation path."))]
	InvalidPath,
	#[cfg_attr(
		feature = "std",
		error("Disallowed SS58 Address Format for this datatype.")
	)]
	FormatNotAllowed,
	#[cfg_attr(feature = "std", error("Password not allowed."))]
	PasswordNotAllowed,
	#[cfg(feature = "std")]
	#[cfg_attr(feature = "std", error("Incorrect URI syntax {0}."))]
	MalformedUri(#[from] AddressUriError),
}

/// Trait used for types that are really just a fixed-length array.
pub trait ByteArray: AsRef<[u8]> + AsMut<[u8]> + for<'a> TryFrom<&'a [u8], Error = ()> {
	/// The "length" of the values of this type, which is always the same.
	const LEN: usize;

	/// A new instance from the given slice that should be `Self::LEN` bytes long.
	fn from_slice(data: &[u8]) -> Result<Self, ()> {
		Self::try_from(data)
	}

	/// Return a `Vec<u8>` filled with raw data.
	fn to_raw_vec(&self) -> Vec<u8> {
		self.as_slice().to_vec()
	}

	/// Return a slice filled with raw data.
	fn as_slice(&self) -> &[u8] {
		self.as_ref()
	}
}

/// Key that can be encoded to/from SS58.
///
/// See <https://docs.substrate.io/v3/advanced/ss58/>
/// for information on the codec.
pub trait Ss58Codec: Sized + AsMut<[u8]> + AsRef<[u8]> + ByteArray {
	/// A format filterer, can be used to ensure that `from_ss58check` family only decode for
	/// allowed identifiers. By default just refuses the two reserved identifiers.
	fn format_is_allowed(f: Ss58AddressFormat) -> bool {
		!f.is_reserved()
	}

	/// Some if the string is a properly encoded SS58Check address.
	fn from_ss58check(s: &str) -> Result<Self, PublicError> {
		Self::from_ss58check_with_version(s).and_then(|(r, v)| match v {
			v if !v.is_custom() => Ok(r),
			v if v == default_ss58_version() => Ok(r),
			v => Err(PublicError::UnknownSs58AddressFormat(v)),
		})
	}

	/// Some if the string is a properly encoded SS58Check address.
	fn from_ss58check_with_version(s: &str) -> Result<(Self, Ss58AddressFormat), PublicError> {
		const CHECKSUM_LEN: usize = 2;
		let body_len = Self::LEN;

		let data = bs58::decode(s)
			.into_vec()
			.map_err(|_| PublicError::BadBase58)?;
		if data.len() < 2 {
			return Err(PublicError::BadLength);
		}
		let (prefix_len, ident) = match data[0] {
			0..=63 => (1, data[0] as u16),
			64..=127 => {
				// weird bit manipulation owing to the combination of LE encoding and missing two
				// bits from the left.
				// d[0] d[1] are: 01aaaaaa bbcccccc
				// they make the LE-encoded 16-bit value: aaaaaabb 00cccccc
				// so the lower byte is formed of aaaaaabb and the higher byte is 00cccccc
				let lower = (data[0] << 2) | (data[1] >> 6);
				let upper = data[1] & 0b00111111;
				(2, (lower as u16) | ((upper as u16) << 8))
			},
			_ => return Err(PublicError::InvalidPrefix),
		};
		if data.len() != prefix_len + body_len + CHECKSUM_LEN {
			return Err(PublicError::BadLength);
		}
		let format = ident.into();
		if !Self::format_is_allowed(format) {
			return Err(PublicError::FormatNotAllowed);
		}

		let hash = ss58hash(&data[0..body_len + prefix_len]);
		let checksum = &hash[0..CHECKSUM_LEN];
		if data[body_len + prefix_len..body_len + prefix_len + CHECKSUM_LEN] != *checksum {
			// Invalid checksum.
			return Err(PublicError::InvalidChecksum);
		}

		let result = Self::from_slice(&data[prefix_len..body_len + prefix_len])
			.map_err(|()| PublicError::BadLength)?;
		Ok((result, format))
	}

	/// Some if the string is a properly encoded SS58Check address, optionally with
	/// a derivation path following.
	fn from_string(s: &str) -> Result<Self, PublicError> {
		Self::from_string_with_version(s).and_then(|(r, v)| match v {
			v if !v.is_custom() => Ok(r),
			v if v == default_ss58_version() => Ok(r),
			v => Err(PublicError::UnknownSs58AddressFormat(v)),
		})
	}

	/// Return the ss58-check string for this key.
	fn to_ss58check_with_version(&self, version: Ss58AddressFormat) -> String {
		// We mask out the upper two bits of the ident - SS58 Prefix currently only supports 14-bits
		let ident: u16 = u16::from(version) & 0b0011_1111_1111_1111;
		let mut v = match ident {
			0..=63 => vec![ident as u8],
			64..=16_383 => {
				// upper six bits of the lower byte(!)
				let first = ((ident & 0b0000_0000_1111_1100) as u8) >> 2;
				// lower two bits of the lower byte in the high pos,
				// lower bits of the upper byte in the low pos
				let second = ((ident >> 8) as u8) | ((ident & 0b0000_0000_0000_0011) as u8) << 6;
				vec![first | 0b01000000, second]
			},
			_ => unreachable!("masked out the upper two bits; qed"),
		};
		v.extend(self.as_ref());
		let r = ss58hash(&v);
		v.extend(&r[0..2]);
		bs58::encode(v).into_string()
	}

	/// Return the ss58-check string for this key.
	fn to_ss58check(&self) -> String {
		self.to_ss58check_with_version(default_ss58_version())
	}

	/// Some if the string is a properly encoded SS58Check address, optionally with
	/// a derivation path following.
	fn from_string_with_version(s: &str) -> Result<(Self, Ss58AddressFormat), PublicError> {
		Self::from_ss58check_with_version(s)
	}
}
