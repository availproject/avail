import { ApiPromise } from 'https://deno.land/x/polkadot@0.2.42/api/mod.ts';
import { waitForBlockInclusion, waitForBlockFinalization } from './misc.ts';
import { KeyringPair } from 'https://deno.land/x/polkadot@0.2.42/keyring/types.ts';

export class Task {
    public api: ApiPromise
    public data: string[]
    public name: string
    public txCount: number
    public finalizedBlockHash = "Error"
    public e2e_measure: PerformanceMeasure
    public internal_measure = 0

    constructor(name: string, api: ApiPromise, data: string[], txCount: number) {
        this.name = name;
        this.api = api;
        this.data = data;
        this.txCount = txCount;
        this.e2e_measure = performance.measure("a");
    }

    async run(stages: Stage[]) {
        for(const stage of stages) {
            await stage.run(this)
        }
    }
}

export interface Stage {
    run(task: Task): Promise<void>
}

export class BlockInclusionStage implements Stage {
    public blockTarget: number
    constructor(blockTarget: number) {
        this.blockTarget = blockTarget;
    }

    async run(task: Task): Promise<void> {
        console.log(`Task ${task.name}: Waiting block inclusion ${this.blockTarget}`);
        await waitForBlockInclusion(task.api, this.blockTarget);
    }
}

export class DataSubmissionStage implements Stage {
    public account: KeyringPair 
    constructor(account: KeyringPair) {
        this.account = account;
    }

    async run(task: Task): Promise<void> {
        console.log(`Task ${task.name}: Submitting data`);

        let nonce = (await task.api.rpc.system.accountNextIndex(this.account.address)).toNumber();
        const txs = [];
        for (let i = 0; i < task.txCount; ++i) {
            txs.push(task.api.tx.dataAvailability.submitData(task.data[i].toString()).signAndSend(this.account, {nonce: nonce}));
            nonce += 1;
        }
        await Promise.all(txs);
    }
}

export class BlockFinalizationStage implements Stage {
    public blockTarget: number
    constructor(blockTarget: number) {
        this.blockTarget = blockTarget;
    }

    async run(task: Task): Promise<void> {
        console.log(`Task ${task.name}: Waiting Block Finalization ${this.blockTarget}`);
        task.finalizedBlockHash = await waitForBlockFinalization(task.api, this.blockTarget);
    }
}

export class PerformanceMeasureStage implements Stage {
    public operation: (task: Task) => Promise<void>
    public stageName: string
    constructor(operation: (task: Task) => Promise<void>,  stageName: string) {
        this.operation = operation;
        this.stageName = stageName;
    }

    async run(task: Task): Promise<void> {
        console.log(`Task ${task.name}: ${this.stageName}`);
        const [start, end] = [`${task.name}_start`, `${task.name}end`];
        performance.mark(start);
        await this.operation(task);
        performance.mark(end);
        task.e2e_measure = performance.measure(this.stageName, start, end);
    }
}

export class DoneStage implements Stage {
    async run(task: Task): Promise<void> {
        console.log(`Task ${task.name}: Done`);
    }
}
