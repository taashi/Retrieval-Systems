extern crate serde_json;
extern crate xml;
extern crate regex;

pub mod bm25;
pub mod tfidf;
pub mod ql;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::env;
use std::process::exit;
use xml::reader::{EventReader, XmlEvent};
use regex::Regex;

use bm25::BM25;
use tfidf::TfIdf;
use ql::QueryLikelihood;

type Index = HashMap<String, IList>;
type IList = HashMap<String, usize>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stop = false;
    let mut i = 2;
    let mut specified_index_file = None;
    let mut specified_query_file = None;
    while i < args.len() {
        match &args[i][..] {
            "--index" => {
                if let Some(path) = args.get(i + 1) {
                    specified_index_file = Some(path.clone());
                } else {
                    eprintln!("Must specify path after --index arg");
                    exit(1);
                };
                i += 2;
            },
            "--queries" => {
                if let Some(path) = args.get(i + 1) {
                    specified_query_file = Some(path.clone());
                } else {
                    eprintln!("Must specify path after --query arg");
                    exit(1);
                };
                i += 2;
            },
            "--stop" => {
                stop = true;
                i += 1;
            },
            other => {
                eprintln!("Unknown arg: {}", other);
                exit(1);
            },
        }
    }

    let index = load_index(specified_index_file);

    let mut retriever: Box<Retriever> = match &args[1][..] {
        "--bm25" => {
            Box::new(BM25::new(index))
        },
        "--tfidf" => {
            Box::new(TfIdf::new(index))
        },
        "--ql" => {
            Box::new(QueryLikelihood::new(index))
        },
        _ => {
            panic!("Must provide argument specifying retriever type, see README.");
        },
    };


    let queries = load_queries(specified_query_file, stop);
    let mut i = 1;
    for query in queries {
        let docs_sorted = retriever.rank(query);
        for j in 0..(100.min(docs_sorted.len())) {
            let e = &docs_sorted[j];
            println!("{} Q0 {} {} {} {}", i, e.0, j, e.1, &args[1][2..]);
        }
        eprintln!("Done with query: {}", i);
        i += 1;
    }
}

fn load_queries(specified_query_file: Option<String>,stop: bool) -> Vec<Vec<String>> {
    let stop_f = File::open("../provided/common_words").expect("common_words file not found");
    let mut stoplist = Vec::new();
    if stop {
        let reader = BufReader::new(stop_f);
        for maybe_line in reader.lines() {
            if let Ok(line) = maybe_line {
                stoplist.push(line);
            }
        }
    }

    let q_file_path = format!("{}/{}",
                              std::env::current_dir().unwrap().display(),
                              specified_query_file.unwrap_or(String::from("../provided/cacm.query.txt")));
    let mut queries = Vec::new();
    let query_f = File::open(q_file_path).expect("Could not open cacm.query.txt");
    let parser = EventReader::new(BufReader::new(query_f));

    let mut query = String::new();
    let mut reading_query = false;

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == "DOCNO" {
                    reading_query = false;
                } else if name.local_name == "DOC" {
                    reading_query = true;
                } else {
                    panic!("Unknown Element in query document!");
                }
            },
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "DOCNO" {
                    reading_query = true;
                } else if name.local_name == "DOC" {
                    let stopped_query_vec: Vec<String> = query.split_whitespace()
                        .map(|s| format_query(String::from(s)))
                        .filter(|s| !stoplist.contains(&s.to_string()))
                        .collect();
                    let stopped_query: String = stopped_query_vec.join(" ");
                    queries.push(stopped_query.clone());
                    query = String::new();
                }
            },
            Ok(XmlEvent::Characters(text)) => {
                if reading_query {
                    query.push(' ');
                    query.push_str(&text);
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
            _ => {},
        }
    }
    queries.iter().map(|q| q.split_whitespace().map(|s| String::from(s)).collect()).collect()
}

fn format_query(query: String) -> String {
    let re_punct_to_space = Regex::new(r"[-,]").unwrap();
    let re_punct_remove = Regex::new(r"[\W&&\S]").unwrap();
    let re_whitespace = Regex::new(r"\s+").unwrap();
    let mut s = query.clone();

    s = String::from(re_punct_to_space.replace_all(s.as_str(), " "));
    s = String::from(re_punct_remove.replace_all(s.as_str(), ""));
    s = String::from(re_whitespace.replace_all(s.as_str(), " "));
    s.to_lowercase()
}


fn load_index(specified_index_file: Option<String>) -> Index {
    let q_file_path = format!("{}/{}",
                              std::env::current_dir().unwrap().display(),
                              specified_index_file.unwrap_or(String::from("../indexer/results/unstemmed_unigram_index.json")));
    let index_f = File::open(q_file_path).expect("Could not load unigram_index.json");
    serde_json::from_reader(BufReader::new(index_f)).unwrap()
}

pub trait Retriever {
    fn rank(&mut self, query: Vec<String>) -> Vec<(String, f64)>;
}
