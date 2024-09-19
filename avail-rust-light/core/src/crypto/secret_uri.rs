// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use core::fmt::Display;

use super::derive_junction::DeriveJunction;
use regex_lite::Regex;
use secrecy::SecretString;
use std::vec::Vec;

// This code is taken from sp_core::crypto::DeriveJunction. The logic should be identical,
// though the code is tweaked a touch!

/// A secret uri (`SURI`) that can be used to generate a key pair.
///
/// The `SURI` can be parsed from a string. The string takes this form:
///
/// ```text
/// phrase/path0/path1///password
/// 111111 22222 22222   33333333
/// ```
///
/// Where:
/// - 1 denotes a phrase or hex string. If this is not provided, the [`DEV_PHRASE`] is used
///   instead.
/// - 2's denote optional "derivation junctions" which are used to derive keys. Each of these is
///   separated by "/". A derivation junction beginning with "/" (ie "//" in the original string)
///   is a "hard" path.
/// - 3 denotes an optional password which is used in conjunction with the phrase provided in 1
///   to generate an initial key. If hex is provided for 1, it's ignored.
///
/// Notes:
/// - If 1 is a `0x` prefixed 64-digit hex string, then we'll interpret it as hex, and treat the hex bytes
///   as a seed/MiniSecretKey directly, ignoring any password.
/// - Else if the phrase part is a valid BIP-39 phrase, we'll use the phrase (and password, if provided)
///   to generate a seed/MiniSecretKey.
/// - Uris like "//Alice" correspond to keys derived from a DEV_PHRASE, since no phrase part is given.
///
/// There is no correspondence mapping between `SURI` strings and the keys they represent.
/// Two different non-identical strings can actually lead to the same secret being derived.
/// Notably, integer junction indices may be legally prefixed with arbitrary number of zeros.
/// Similarly an empty password (ending the `SURI` with `///`) is perfectly valid and will
/// generally be equivalent to no password at all.
///
/// # Examples
///
/// Parse [`DEV_PHRASE`] secret URI with junction:
///
/// ```
/// # use subxt_signer::{SecretUri, DeriveJunction, DEV_PHRASE, ExposeSecret};
/// # use std::str::FromStr;
/// let suri = SecretUri::from_str("//Alice").expect("Parse SURI");
///
/// assert_eq!(vec![DeriveJunction::from("Alice").harden()], suri.junctions);
/// assert_eq!(DEV_PHRASE, suri.phrase.expose_secret());
/// assert!(suri.password.is_none());
/// ```
///
/// Parse [`DEV_PHRASE`] secret URI with junction and password:
///
/// ```
/// # use subxt_signer::{SecretUri, DeriveJunction, DEV_PHRASE, ExposeSecret};
/// # use std::str::FromStr;
/// let suri = SecretUri::from_str("//Alice///SECRET_PASSWORD").expect("Parse SURI");
///
/// assert_eq!(vec![DeriveJunction::from("Alice").harden()], suri.junctions);
/// assert_eq!(DEV_PHRASE, suri.phrase.expose_secret());
/// assert_eq!("SECRET_PASSWORD", suri.password.unwrap().expose_secret());
/// ```
///
/// Parse [`DEV_PHRASE`] secret URI with hex phrase and junction:
///
/// ```
/// # use subxt_signer::{SecretUri, DeriveJunction, DEV_PHRASE, ExposeSecret};
/// # use std::str::FromStr;
/// let suri = SecretUri::from_str("0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a//Alice").expect("Parse SURI");
///
/// assert_eq!(vec![DeriveJunction::from("Alice").harden()], suri.junctions);
/// assert_eq!("0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a", suri.phrase.expose_secret());
/// assert!(suri.password.is_none());
/// ```
pub struct SecretUri {
	/// The phrase to derive the private key.
	///
	/// This can either be a 64-bit hex string or a BIP-39 key phrase.
	pub phrase: SecretString,
	/// Optional password as given as part of the uri.
	pub password: Option<SecretString>,
	/// The junctions as part of the uri.
	pub junctions: Vec<DeriveJunction>,
}

impl core::str::FromStr for SecretUri {
	type Err = SecretUriError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let cap = secret_phrase_regex()
			.captures(s)
			.ok_or(SecretUriError::InvalidFormat)?;

		let junctions = junction_regex()
			.captures_iter(&cap["path"])
			.map(|f| DeriveJunction::from(&f[1]))
			.collect::<Vec<_>>();

		let phrase = cap.name("phrase").map(|r| r.as_str()).unwrap_or(DEV_PHRASE);
		let password = cap.name("password");

		Ok(Self {
			phrase: SecretString::from_str(phrase).expect("Returns infallible error; qed"),
			password: password.map(|v| {
				SecretString::from_str(v.as_str()).expect("Returns infallible error; qed")
			}),
			junctions,
		})
	}
}

/// This is returned if `FromStr` cannot parse a string into a `SecretUri`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SecretUriError {
	/// Parsing the secret URI from a string failed; wrong format.
	InvalidFormat,
}

impl Display for SecretUriError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			SecretUriError::InvalidFormat => write!(f, "Invalid secret phrase format"),
		}
	}
}

#[cfg(feature = "std")]
impl std::error::Error for SecretUriError {}

/// Interpret a phrase like:
///
/// ```text
/// foo bar wibble /path0/path1///password
/// 11111111111111 222222222222   33333333
/// ```
/// Where 1 is the phrase, 2 the path and 3 the password.
/// Taken from `sp_core::crypto::SECRET_PHRASE_REGEX`.
fn secret_phrase_regex() -> Regex {
	static SECRET_PHRASE_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
	SECRET_PHRASE_REGEX
		.get_or_init(|| {
			Regex::new(r"^(?P<phrase>[\d\w ]+)?(?P<path>(//?[^/]+)*)(///(?P<password>.*))?$")
				.unwrap()
		})
		.clone()
}

/// Interpret a part of a path into a "junction":
///
/// ```text
/// //foo/bar/wibble
///  1111 222 333333
/// ```
/// Where the numbers denote matching junctions.
///
/// The leading "/" deliminates each part, and then a "/" beginning
/// a path piece denotes that it's a "hard" path. Taken from
/// `sp_core::crypto::JUNCTION_REGEX`.
fn junction_regex() -> Regex {
	static JUNCTION_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
	JUNCTION_REGEX
		.get_or_init(|| Regex::new(r"/(/?[^/]+)").unwrap())
		.clone()
}

/// The root phrase for our publicly known keys.
pub const DEV_PHRASE: &str =
	"bottom drive obey lake curtain smoke basket hold race lonely fit walk";
