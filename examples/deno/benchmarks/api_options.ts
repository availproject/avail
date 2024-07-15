export const API_RPC = {
	kate: {
		queryRows: {
			description: "",
			params: [
				{
					name: "rows",
					type: "Vec<u32>",
				},
				{
					name: "at",
					type: "Hash",
					isOptional: true,
				},
			],
			type: "Vec<GRow>",
		},
		queryProof: {
			description: "Generate the kate proof for the given `cells`",
			params: [
				{
					name: "cells",
					type: "Vec<Cell>",
				},
				{
					name: "at",
					type: "Hash",
					isOptional: true,
				},
			],
			type: "Vec<GDataProof>",
		},
		blockLength: {
			description: "Get Block Length",
			params: [
				{
					name: "at",
					type: "Hash",
					isOptional: true,
				},
			],
			type: "BlockLength",
		},
		queryDataProof: {
			description: "Generate the data proof for the given `transaction_index`",
			params: [
				{
					name: "transaction_index",
					type: "u32",
				},
				{
					name: "at",
					type: "Hash",
					isOptional: true,
				},
			],
			type: "ProofResponse",
		},
		queryRowsMetrics: {
			description: "",
			params: [
				{
					name: "rows",
					type: "Vec<u32>",
				},
				{
					name: "at",
					type: "Hash",
					isOptional: true,
				},
			],
			type: "(Vec<GRow>, u128)",
		},
		queryProofMetrics: {
			description: "Generate the kate proof for the given `cells`",
			params: [
				{
					name: "cells",
					type: "Vec<Cell>",
				},
				{
					name: "at",
					type: "Hash",
					isOptional: true,
				},
			],
			type: "(Vec<GDataProof>, u128)",
		},
		blockLengthMetrics: {
			description: "Get Block Length",
			params: [
				{
					name: "at",
					type: "Hash",
					isOptional: true,
				},
			],
			type: "(BlockLength, u128)",
		},
		queryDataProofMetrics: {
			description: "Generate the data proof for the given `index`",
			params: [
				{
					name: "transaction_index",
					type: "u32",
				},
				{
					name: "at",
					type: "Hash",
					isOptional: true,
				},
			],
			type: "(ProofResponse, u128)",
		},
	},
};
