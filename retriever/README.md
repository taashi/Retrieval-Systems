Build Requirements:
This implementation was written in Rust using the Nightly compiler.
It can be installed using `rustup`: https://www.rustup.rs/
ensure that you are using the nightly toolchain

`rustup toolchain install nightly; rustup default nightly`

Build instructions:
Rust comes with the Cargo build tool, the project can be build using
`cargo build`

Running:

Once Cargo completes, a binary is placed in `./target/debug/project_retriever`
This binary runs different tools depending on the command line arguments.
Only one argument can be supplied to the binary either by running:

`cargo run -- <retriever> <args>`

or

`./target/debug/project_retriever <retriever> <args>`

Results for the specified run are written to stdout, so it is recommended
that you redirect the output to a file, for example:

`cargo run -- <retriever> <args> > myresults.txt`

where retriever can be one of:

* `--bm25`
* `--tfidf`
* `--ql`

args are optional specified
* `--index` <index-file-path> : Defaults to ../stemmed_unigram.json
* `--queries` <query_file_path> : Defaults to ../cacm.query.text. Query file must be in the format

```<DOC> <DOCNO> N </DOCNO> query... </DOC> ...```
* `--stop` Specify if you'd like to use stopping using ../common_words as a stop list

Files:

`./results/*` Result files with ranked documents for each retrieval method as well as
stemmed and stopped results for bm25, tfidf, and query likelihood.


`src/bm25.rs` Implementation for the bm25 retrieval model

`src/tfidf.rs` Implementation for the tfidf retrieval model

`src/ql.rs` Implementation for the query likelihood retrieval model

`src/main.rs` Main entry point for the program

`src/BM25_Pseudo.py` Python code for Query Expansion using Pseudo relevance feedback

`Cargo.toml` dependency information used by build system

`README` this file

Libraries used:

* regex: : Regex library used for text processing
* serde  : Serialization library used for writing json files
* xml-rs : XML parser used for parsing the query format
