import { ApiPromise, Keyring, WsProvider } from 'https://deno.land/x/polkadot@0.2.42/api/mod.ts';
import { API_RPC, API_TYPES, API_EXTENSIONS } from './../api_options.ts'
import { prepareData } from './misc.ts';
import { BlockFinalizationStage, BlockInclusionStage, PerformanceMeasureStage, DataSubmissionStage, DoneStage, Task } from './task.ts';

const api = await ApiPromise.create({ provider: new WsProvider("ws://127.0.0.1:9944"), rpc: API_RPC, types: API_TYPES, signedExtensions: API_EXTENSIONS  });
const alice = new Keyring({type: 'sr25519'}).addFromUri("//Alice");

console.log("Preparing data...")
const txCount = 100;
const data = prepareData(txCount);

const targetBlockNumber = (await api.rpc.chain.getHeader()).number.toNumber() + 1;
const tasks: Task[] = [];
const jobs = [];
const jobCount = 10;

for(let i = 0; i < jobCount; ++i) {
    const task = new Task(`${i}`, api, data, txCount);

    const customStage = new PerformanceMeasureStage(async (task) => await task.api.rpc.kate.queryDataProof(1, task.finalizedBlockHash), "Querying Data Proof");
    const stages = [new BlockInclusionStage(targetBlockNumber + i), new DataSubmissionStage(alice), new BlockFinalizationStage(targetBlockNumber + 1 + i), customStage, new DoneStage()];
    jobs.push(task.run(stages));
    tasks.push(task);
}

await Promise.all(jobs);

const durations = tasks.map((t) => t.measure?.duration);
const totalTime = durations.reduce((pv, cv) => pv += cv);
console.log(durations);
console.log(`Total time: ${totalTime}; Average time: ${totalTime / jobCount}`);

Deno.exit(0);