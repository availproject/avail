import { ApiPromise, Keyring, WsProvider } from 'https://deno.land/x/polkadot@0.2.42/api/mod.ts';
import { API_TYPES, API_EXTENSIONS } from './../api_options.ts'
import { API_RPC } from './api_options.ts'
import { prepareData } from './misc.ts';
import { BlockFinalizationStage, BlockInclusionStage, PerformanceMeasureStage, DataSubmissionStage, DoneStage, Task } from './task.ts';
import config from './config.ts';

const api = await ApiPromise.create({ provider: new WsProvider(config.endpoint), rpc: API_RPC, types: API_TYPES, signedExtensions: API_EXTENSIONS  });
const keyring = new Keyring({type: 'sr25519'}).addFromUri(config.seed);

console.log("Preparing data...")
const txCount = config.txCount;
const data = prepareData(txCount);

const targetBlockNumber = (await api.rpc.chain.getHeader()).number.toNumber() + 1;
const tasks: Task[] = [];
const jobs = [];
const jobCount = config.jobCount;

for(let i = 0; i < jobCount; ++i) {
    const task = new Task(`${i}`, api, data, txCount);
    const customStage = new PerformanceMeasureStage(async (task) => {
        const res = await task.api.rpc.kate.queryDataProofV2Metrics(1, task.finalizedBlockHash);
        task.internal_measure = res[1].toNumber() / 1000;
    }, "Querying Data Proof");
    const stages = [new BlockInclusionStage(targetBlockNumber + i), new DataSubmissionStage(keyring), new BlockFinalizationStage(targetBlockNumber + 1 + i), customStage, new DoneStage()];
    
    jobs.push(task.run(stages));
    tasks.push(task);
}

await Promise.all(jobs);

const e2eDurations = tasks.map((t) => t.e2e_measure?.duration);
const e2eTotalTime = e2eDurations.reduce((pv, cv) => pv += cv);
const internalDurations = tasks.map((t) => t.internal_measure);
const internalTotalTime = internalDurations.reduce((pv, cv) => pv += cv);

const zip = e2eDurations.map((v, i) => [v, internalDurations[i]]);
console.log(zip);

console.log(`Total E2E time: ${e2eTotalTime}; Average E2E time: ${e2eTotalTime / jobCount}`);
console.log(`Total Internal time: ${internalTotalTime}; Average Internal time: ${internalTotalTime / jobCount}`);

Deno.exit(0);
