Build Requirements:
This implementation was written in Rust using the Nightly compiler.
It can be installed using `rustup`: https://www.rustup.rs/
ensure that you are using the nightly toolchain

`rustup toolchain install nightly; rustup default nightly`

Build instructions:
Rust comes with the Cargo build tool, the project can be build using
`cargo build`

Running:
Once Cargo completes, a binary is placed in `./target/debug/evaluation`
This binary runs different tools depending on the command line arguments.
Only one argument can be supplied to the binary either by running:
`cargo run -- <path_to_results>`
or
`./target/debug/evaluation <path_to_results>`

path_to_results should point to one of the files in
the ../retrieval/results directory.

Results for the specified run are written in csv format to stdout,
so it is recommended that you redirect the output to a file, for example:

`cargo run -- ../retrieval/results/bm25.txt > bm25-results.csv`

The output is csv formatted text including all retrieval metrics.

Files:
./resuts/* Result files with all evaluation metrics for each run.

src/main.rs Main entry point to the program
src/map.rs  Implementation for MAP
src/mrr.rs  Implementation for MRR
src/pak.rs  Implementation for P@K
src/pr.rs   Implementation for Precision and Recall

Cargo.toml:          dependency information used by build system
README:              this file

Libraries used:
none