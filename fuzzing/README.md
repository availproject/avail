# AFL fuzzing framework

This crate contains the harness code required for AFL to run fuzz tests on individual functions in Avail repo.

## AFL.rs setup

Run `cargo install afl` on target machine

## Target binary build

Run `cargo afl build` inside the fuzzing crate. This command builds an instrumented library needed for the fuzzing. Testing multiple functions will require multiple harnesses which will be built as different target executables found in the Cargo.toml.

## Execution

1. Create input and output directories. The input directory contains the initial fuzz seeds (inputs) which will be used by AFL as input data for the harness. Output directory contains folders with stated data generated during the fuzzing, as well as samples that caused crashes and timeouts.

2. Populate input directory with sample inputs

3. Run `cargo afl fuzz -i in/ -o out/ ../../target/debug/target_name`, with target name being the name of the binary containing the target function. Note that a single AFL instance runs on a single core, without parallel execution capabilities. In order to achieve far greater fuzzing speeds, a [distributed system](https://aflplus.plus/docs/parallel_fuzzing/) is needed.

## TODO

- analyze most interesting target functions and implement needed harnesses
- implement a deployment strategy for scaling fuzzer capacity per function, related to the available hardware resources (i.e. CPU cores)

## Further reading

- https://github.com/google/AFL
- AFL.rs uses [AFL++](https://github.com/AFLplusplus/AFLplusplus)
- AFL.rs setup [tutorial](https://rust-fuzz.github.io/book/afl/tutorial.html)
- [Status screen explaination](https://lcamtuf.coredump.cx/afl/status_screen.txt)

