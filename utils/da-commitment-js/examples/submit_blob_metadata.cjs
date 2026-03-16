const { ApiPromise, WsProvider } = require("@polkadot/api");
const { Keyring } = require("@polkadot/keyring");
const { u8aToHex } = require("@polkadot/util");
const { cryptoWaitReady, keccakAsU8a } = require("@polkadot/util-crypto");
const { build_commitments_js } = require("../pkg_node");

const WS_URL = "ws://91.98.84.164:8546";
const FRI_PARAMS_VERSION = 0;
const APP_ID = 0;
const ERA_PERIOD = 32;
const TEXT = "Hello World!";

function toBase64(bytes) {
	return Buffer.from(bytes).toString("base64");
}

function toNumber(codec, label) {
	const value = codec.toNumber();

	if (!Number.isSafeInteger(value)) {
		throw new Error(`${label} does not fit in a safe JavaScript integer`);
	}

	return value;
}

async function main() {
	await cryptoWaitReady();

	const provider = new WsProvider(WS_URL);
	const api = await ApiPromise.create({ provider });

	try {
		const keyring = new Keyring({ type: "sr25519" });
		const alice = keyring.addFromUri("//Alice");

		const text = TEXT;
		const encoder = new TextEncoder();
		const blob = encoder.encode(text);
		const blobHash = keccakAsU8a(blob);
		const babeRandomnessValue = await api.query.babe.randomness();
		const babeRandomness = new Uint8Array(babeRandomnessValue.toU8a());

		if (babeRandomness.length !== 32) {
			throw new Error(
				`Babe randomness is unavailable or invalid, expected 32 bytes and got ${babeRandomness.length}`
			);
		}

		const metadata = build_commitments_js(
			blob,
			FRI_PARAMS_VERSION,
			babeRandomness,
			blobHash
		);
		const nonce = toNumber(await api.rpc.system.accountNextIndex(alice.address), "nonce");
		const finalizedHash = await api.rpc.chain.getFinalizedHead();
		const finalizedHeader = await api.rpc.chain.getHeader(finalizedHash);
		const era = api.registry.createType("ExtrinsicEra", {
			current: toNumber(finalizedHeader.number, "finalizedHeader.number"),
			period: ERA_PERIOD,
		});

		const tx = api.tx.dataAvailability.submitBlobMetadata(
			APP_ID,
			u8aToHex(blobHash),
			blob.length,
			u8aToHex(metadata.commitment),
			u8aToHex(metadata.eval_point_seed),
			u8aToHex(metadata.eval_claim)
		);

		const signed = await tx.signAsync(alice, {
			blockHash: finalizedHash,
			era,
			genesisHash: api.genesisHash,
			nonce,
			runtimeVersion: api.runtimeVersion,
		});

		await provider.send("blob_submitBlob", [
			toBase64(signed.toU8a()),
			toBase64(blob),
		]);
		console.log("blob_submitBlob submitted successfully");
		console.log(`text: ${text}`);
		console.log(`blobHash: ${u8aToHex(blobHash)}`);
		console.log(`commitment: ${u8aToHex(metadata.commitment)}`);
		console.log(`evalPointSeed: ${u8aToHex(metadata.eval_point_seed)}`);
		console.log(`evalClaim: ${u8aToHex(metadata.eval_claim)}`);
		console.log(`appId: ${APP_ID}`);
		console.log(`nonce: ${nonce}`);
		console.log(`txHash: ${signed.hash.toHex()}`);
	} finally {
		await api.disconnect();
	}
}

main().catch((error) => {
	console.error(error);
	process.exitCode = 1;
});
