const { ApiPromise, WsProvider } = require('@polkadot/api');
const yargs = require('yargs');


async function createApi(endpoint: string) {
	// Initialise the provider to connect to the local node
	const provider = new WsProvider(endpoint);

	// Create the API and wait until ready
	return ApiPromise.create({ 
		provider,
		types: {
			DataLookup: {
				size: 'u32',
				index: 'Vec<(u32,u32)>'
			},
			KateExtrinsicRoot: {
				hash: 'Hash',
				commitment: 'Vec<u8>',
				rows: 'u16',
				cols: 'u16'
			},
			KateHeader: {
				parentHash: 'Hash',
				number: 'Compact<BlockNumber>',
				stateRoot: 'Hash',
				extrinsicsRoot: 'KateExtrinsicRoot',
				digest: 'Digest',
				appDataLookup: 'DataLookup'
			},
			Header: 'KateHeader',
			AppId: 'u32',
			CheckAppId: {
				extra: {
					appId: 'u32', 
				},
				types: {}
			}
		},
		signedExtensions: {
			CheckAppId: {
				extrinsic: {
					appId: 'u32'
				},
				payload: {}
			},
		},
	});
}

async function cli_arguments() {
	return yargs(process.argv.slice(2)).options({
		e: { 
			description: 'WSS endpoint',
			alias: 'endpoint',
			type: 'string', 
			default: 'wss://polygon-da-explorer.matic.today/ws'
		},
		b: {
			description: 'Block ID',
			alias: 'block_id',
			type: 'number',
			demandOption: true,
		},
		i: {
			description: 'Extrinsic Index',
			alias: 'extrinsic_index',
			type: 'number',
		}
	}).argv
}

async function main () {
	const argv = await cli_arguments();

	const api = await createApi(argv.e); 

	const blockNumber = argv.b;
	const extrinsicIndex = argv.i;

	const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
	const signedBlock = await api.rpc.chain.getBlock(blockHash);

	let extrinsics = signedBlock.block.extrinsics;
	if (extrinsicIndex !== undefined) {
		extrinsics = extrinsics.filter( function(val:any, idx:number, array:any) {
			return idx == extrinsicIndex
		});
	}


	console.log(`Block Hash: ${blockHash}`);
	console.log(`Extrinsic:`);
	extrinsics.forEach((ex: any,index: number, array: any) => {
		let ex_str = JSON.stringify(ex.toHuman(), null, 2);
		if (extrinsicIndex) {
			console.log(`${ex_str}`);
		}
		else {
			console.log(`${index}: ${ex_str}`);
		}
	});

	process.exit(0);
}

main().catch( err => {
	console.error(err);
	process.exit(-1);
});

