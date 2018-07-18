extern crate serde_json;
extern crate select;
extern crate regex;

pub mod index;
pub mod parse;

use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::env;

static STEMMED_DOC_PATH:&str = "../provided/cacm_stem.txt";
static UNSTEMMED_DOC_PATH: &str = "../provided/docs/";

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "--index-stemmed" => {
            println!("Generating index for all files in {}", STEMMED_DOC_PATH);
            let mut indexer = index::Indexer::new();
            let doc_file = File::open(STEMMED_DOC_PATH).expect("Document file missing");
            let reader = BufReader::new(doc_file);

            let mut doc_id = 1;

            let mut doc = String::new();
            let mut first_line = true;
            for maybe_line in reader.lines() {
                // Have to skip the first line because there isn't a document
                // before it to write to the index
                if first_line {
                    first_line = false;
                    continue;
                }
                if let Ok(line) = maybe_line {
                    // Document separator lines have the format "# 1" for doc_id 1
                    // Assume documents are in order by numerical id
                    // Write the previous document to the index, stripping traling numbers
                    if line.starts_with("#") {
                        let digits_stripped = parse::remove_trailing_numbers(doc);
                        indexer.index(digits_stripped, format!("{}", doc_id));
                        doc = String::new();
                        doc_id += 1;
                    } else {
                        // Add line to the document
                        doc.push(' ');
                        doc.push_str(&line);
                    }
                }
            }

            let un_index_f = File::create("./results/stemmed_unigram_index.json").unwrap();
            indexer.write_un_index(un_index_f);

        },
        "--index-unstemmed" => {
            let paths = fs::read_dir(UNSTEMMED_DOC_PATH).expect("../docs/ directory is missing");
            let no_case_folding = args.iter().any(|s| *s == String::from("--no-case-folding"));
            let no_punct_removal = args.iter().any(|s| *s == String::from("--no-punct-removal"));
            println!("Parsing all files in {}. Case folding: {} Punctuation removal: {}",
                     UNSTEMMED_DOC_PATH,
                     !no_case_folding,
                     !no_punct_removal);

            let mut indexer = index::Indexer::new();
            for path in paths {
                if let Ok(p) = path {
                    let f = File::open(p.path()).unwrap();
                    let parsed_f_path = p.path()
                        .to_str().unwrap()
                        .replace("docs", "parsed")
                        .replace(".html", ".txt");
                    let mut parsed = File::create(parsed_f_path.clone())
                        .expect("Could not create file for parsed document ");
                    let out = parse::parse(f, !no_case_folding, !no_punct_removal);
                    parsed.write(out.as_bytes()).unwrap();

                    let start_idx = parsed_f_path.find("CACM-").unwrap() + 5;
                    let end_idx = parsed_f_path.find(".txt").unwrap();
                    let doc_id = (&parsed_f_path[start_idx..end_idx]).parse::<usize>().unwrap();
                    indexer.index(out, format!("{}", doc_id));

                }
            }
            let un_index_f = File::create("./results/unstemmed_unigram_index.json").unwrap();
            indexer.write_un_index(un_index_f);

        },
        _ => {
            println!("Invalid arguments");
        },
    }
}
