# AFL fuzzing framework

This crate contains the harness code required for AFL to run fuzz tests on individual functions in Avail repo.

## Setup

Run the setup.sh script on target machine, that will install Rust dependencies and AFL.rs. 

## Target binary build

Run `cargo afl build` inside the fuzzing crate. This command builds an instrumented library needed for the fuzzing. Testing multiple functions will require multiple harnesses which will be built as different target executables found in the Cargo.toml.

## Simple Execution

1. Create input and output directories. The input directory contains the initial fuzz seeds (inputs) which will be used by AFL as input data for the harness. Output directory contains folders with stated data generated during the fuzzing, as well as samples that caused crashes and timeouts.

2. Populate input directory with sample inputs

3. Run `cargo afl fuzz -i in/ -o out/ ../../target/debug/target_name`, with target name being the name of the binary containing the target function. Note that a single AFL instance runs on a single core, without parallel execution capabilities. In order to achieve far greater fuzzing speeds, a [distributed system](https://aflplus.plus/docs/parallel_fuzzing/) is needed.

## Parallel Execution

Run the run_afl.sh script inside the fuzzing directory with `./run_afl.sh target_binary num_of_instances` command. The script will instantiate a single master node with multiple slave nodes, with no oversight needed. 

Use the AFL whatsup tool to monitor the execution. Use SIGKILL with individual fuzzer PIDs (provided by the `whatsup` tool) to kill the fuzzers when needed.

## Notes

- Lower stability in the item geometry is to be expected when encountering code with some built-in randomness
- Harness function needs to be crafted in a way to optimize the AFLs chances of finding bugs (i.e. delivering reasonable extrinsic format)
- AFL whatsup too (`cargo afl whatsup path/to/output/dir`) can be used to check the status of the fuzzers running in background
- For better results in parallel fuzzing, a single master (-M) instance is required to perform deterministic mutations

## TODO

- [x] implement a simple deployment script for parallel fuzzing
- [ ] implement a simple crash/hangs signaling script with a signaling mechanism (i.e. upload the sample to S3)
- [ ] analyze most interesting target functions and implement needed harnesses
- [ ] implement a deployment strategy for scaling fuzzer capacity per function, related to the available hardware resources (i.e. CPU cores)
- [ ] implement a CICD fuzzing strategy

## Further reading

- https://github.com/google/AFL
- AFL.rs uses [AFL++](https://github.com/AFLplusplus/AFLplusplus)
- AFL.rs setup [tutorial](https://rust-fuzz.github.io/book/afl/tutorial.html)
- [Status screen explaination](https://lcamtuf.coredump.cx/afl/status_screen.txt)

