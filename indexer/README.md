Build Requirements:
This implementation was written in Rust using the Nightly compiler.
It can be installed using `rustup`: https://www.rustup.rs/
ensure that you are using the nightly toolchain

`rustup toolchain install nightly; rustup default nightly`

Build instructions:
Rust comes with the Cargo build tool, the project can be build using
`cargo build`

Running:
Once Cargo completes, a binary is placed in `./target/debug/project_indexer`
This binary runs different tools depending on the command line arguments.
Only one argument can be supplied to the binary either by running:
`cargo run -- <arg>`
or
`./target/debug/project_indexer <arg>`

--index-stemmed
  - Created an index for the ../cacm_stem.txt file, writing the index to ../stemmed_unigram_index.json
--index-unstemmed
  - Created an index for all files in ../docs, writing the index to ../unstemmed_unigram_index.json


Files:
results/stemmed_unigram_index.json   Unigram term frequency index for the `cacm_stem.txt` documents
results/unstemmed_unigram_index.json Unigram term frequency index for the documents in `./docs`

src/index.rs Code for indexing documents and generating the output files.
src/main.rs  Main runner file.
src/parse.rs Code for parsing the html output, case folding, punctuation and whitespace removal.

Cargo.toml: dependency information used by build system
README:     this file

Both index files have the format:
```
{
    "term": {
        "doc_id": tf,
        ...
    },
    ...
}
```


Libraries used:
select : A Rust library for parsing and querying HTML
regex: : Regex library used for text processing
serde  : Serialization library used for writing json files
