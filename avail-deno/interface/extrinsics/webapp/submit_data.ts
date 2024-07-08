import { ExecutionState, SharedState } from "./shared_state.ts";

export class SubmitDataView {
	private sharedState: SharedState;
	private executionState: ExecutionState = ExecutionState.Idle;

	constructor(sharedState: SharedState) {
		this.sharedState = sharedState;
	}

	async execute(data: string): Promise<string | null> {
		if (this.executionState != ExecutionState.Idle) {
			return null;
		}

		this.executionState = ExecutionState.Busy;
		const result = await this.sharedState.sdk.tx.dataAvailability.submitData(data, this.sharedState.waitFor, this.sharedState.account);
		this.executionState = ExecutionState.Idle;
		if (result.isErr) {
			return result.reason;
		} else {
			return `TxHash: ${result.txHash}<br> BlockHash: ${result.blockHash}<br> Event: { who: ${result.event.who}, dataHash: ${result.event.dataHash} }<br> TX Data: ${result.txData}`;
		}
	}

	static generateHtml(): string {
		return Deno.readTextFileSync("./submit_data.html");
	}
}
