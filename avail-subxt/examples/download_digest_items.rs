use std::{fmt::Write, str::from_utf8};

use anyhow::Result;
use async_std::{fs::File, io::BufWriter, prelude::*};
use avail_subxt::{
	build_client,
	primitives::{babe, grandpa},
	Opts,
};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use serde_json::{json, Value};
use structopt::StructOpt;
use subxt::{
	config::substrate::{ConsensusEngineId, DigestItem},
	ext::codec::Decode,
};

fn digests_to_json(digests: Vec<DigestItem>) -> Value {
	Value::Array(digests.into_iter().map(digest_to_json).collect::<Vec<_>>())
}

fn digest_to_json(digest: DigestItem) -> Value {
	match digest {
		DigestItem::Consensus(id, data) => json!({
			"id": from_utf8(&id).unwrap_or_default(),
			"consensus": consensus_log_to_json(id, data),
		}),
		DigestItem::PreRuntime(id, data) => json!({
			"id": from_utf8(&id).unwrap_or_default(),
			"preRuntime": pre_runtime_to_json(id, data),
		}),
		DigestItem::Seal(id, data) => json!({
			"id": from_utf8(&id).unwrap_or_default(),
			"seal": seal_log_to_json(id, data),
		}),
		DigestItem::Other(data) => json!({ "Other": hex::encode(data) }),
		DigestItem::RuntimeEnvironmentUpdated => json!("RuntimeEnvironmentUpdated"),
	}
}

fn consensus_log_to_json(id: ConsensusEngineId, encoded: Vec<u8>) -> Value {
	let log_json = match &id {
		b"BABE" => {
			let log = babe::ConsensusLog::decode(&mut encoded.as_slice()).unwrap();
			serde_json::to_string(&log).unwrap()
		},
		b"FRNK" => {
			let log = grandpa::ConsensusLog::<u32>::decode(&mut encoded.as_slice()).unwrap();
			serde_json::to_string(&log).unwrap()
		},
		_ => hex::encode(&encoded),
	};

	serde_json::from_str(&log_json).unwrap()
}

fn seal_log_to_json(id: ConsensusEngineId, encoded: Vec<u8>) -> Value {
	let log_json = match &id {
		b"BABE" => {
			let log = babe::AuthoritySignature::decode(&mut encoded.as_slice()).unwrap();
			json!({ "AuthoritySignature": log }).to_string()
		},
		_ => hex::encode(&encoded),
	};

	serde_json::from_str(&log_json).unwrap()
}

fn pre_runtime_to_json(id: ConsensusEngineId, encoded: Vec<u8>) -> Value {
	let log_json = match &id {
		b"BABE" => {
			let log = babe::PreDigest::decode(&mut encoded.as_slice()).unwrap();
			serde_json::to_string(&log).unwrap()
		},
		_ => hex::encode(&encoded),
	};

	serde_json::from_str(&log_json).unwrap()
}

const OUTPUT_PATH: &str = "/tmp/header.json";

#[async_std::main]
async fn main() -> Result<()> {
	let args = Opts::from_args();
	let (client, _) = build_client(args.ws, args.validate_codegen).await?;

	// Get best block
	let best_block = client
		.rpc()
		.block(None)
		.await?
		.expect("Best block always exists .qed");

	let best_block_num = best_block.block.header.number;

	// Create the Progress Bar
	let pb = ProgressBar::new(best_block_num.into());
	pb.set_style(
		ProgressStyle::with_template(
			"[{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len}",
		)
		.unwrap()
		.with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
			write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
		})
		.progress_chars("#>-"),
	);

	// Create the output file
	pb.println(format!("Saving Digest logs into {OUTPUT_PATH}"));
	let mut out = BufWriter::new(File::create(OUTPUT_PATH).await?);
	out.write(b"[\n").await?;

	// Load all headers and decode each digest item.
	for block_num in 1..=best_block_num {
		let block_hash = client
			.rpc()
			.block_hash(Some(block_num.into()))
			.await?
			.unwrap();
		let header = client.rpc().header(Some(block_hash)).await?.unwrap();

		let digests = header
			.digest
			.logs
			.into_iter()
			// .filter_map(|encoded| DigestItem::decode(encoded).ok())
			.collect::<Vec<_>>();

		let header_json_value = json!({
			"number": block_num,
			"hash": block_hash,
			"digest": digests_to_json(digests),
		});

		pb.inc(1);
		let header_json = serde_json::to_string_pretty(&header_json_value)?;
		// println!("{header_json}");
		out.write(header_json.as_bytes()).await?;
		out.write(b",\n").await?;
	}

	out.write(b"\n]\n").await?;
	out.flush().await?;
	pb.finish_with_message("done");

	Ok(())
}
