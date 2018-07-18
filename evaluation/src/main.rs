pub mod map;
pub mod mrr;
pub mod pak;
pub mod pr;

use map::Map;
use mrr::Mrr;
use pak::Pak;
use pr::PrecisionAndRecall;

use std::collections::{HashSet, HashMap};
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::process;

fn main() {
    let relevant_docs = get_relevance_info();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Must specify results file to load from");
        process::exit(1);
    }

    let result_name = &args[1][..];

    println!("\"Retrieval Model\", \"Evaluation Metric\", \"Value\"");
    let results = get_results(String::from(result_name.clone()));
    println!("\"{}\", \"MAP\", \"{}\"", result_name, Map::calc(&results, &relevant_docs));
    println!("\"{}\", \"MRR\", \"{}\"", result_name, Mrr::calc(&results, &relevant_docs));

    print!("\n");
    println!("\"Retrieval Model\", \"Evaluation Metric\", \"Query\", \"Value\"");
    for (query_id, pak) in Pak::calc(5, &results, &relevant_docs) {
        println!("\"{}\", \"P@5\", \"{}\", \"{}\"", result_name, query_id, pak);
    }
    for (query_id, pak) in Pak::calc(20, &results, &relevant_docs) {
        println!("\"{}\", \"P@20\", \"{}\", \"{}\"", result_name, query_id, pak);
    }

    print!("\n");
    println!("\"Retrieval Model\", \"Query\", \"Rank\", \"Precision\", \"Recall\"");
    for (query_id, rank, precision, recall) in PrecisionAndRecall::calc(&results, &relevant_docs) {
        println!("\"{}\", \"{}\", \"{}\", \"{}\", \"{}\"", result_name, query_id, rank, precision, recall);
    }


    print!("\n\n");

}

/// Gets a mapping from query to list of retrieved
/// documents in order of relevance.
fn get_results(file_name: String) -> HashMap<usize, Vec<usize>> {
    let path = format!("{}/{}", env::current_dir().unwrap().display(), file_name);
    let results_f = File::open(path).expect("Results file not found");
    let results_reader = BufReader::new(results_f);
    let mut results: HashMap<usize, Vec<usize>> = HashMap::new();

    for m_line in results_reader.lines() {
        if let Ok(line) = m_line {
            let parts: Vec<String> = line.split_whitespace().map(|s| String::from(s)).collect();
            let query_id = parts.get(0).expect("Relevance line is not formatted correctly").parse::<usize>().unwrap();
            let doc_id = parts.get(2).expect("Relevance line is not formatted correctly").parse::<usize>().unwrap();
            let docs = results.entry(query_id).or_insert(Vec::new());
            docs.push(doc_id.clone());
        }
    }

    results
}

/// Gets a mapping from query to set of relevant documents
fn get_relevance_info() -> HashMap<usize, HashSet<usize>> {
    let rel_file = File::open("../cacm.rel.txt").expect("../cacm.rel.txt not found");
    let rel_reader = BufReader::new(rel_file);
    let mut relevant_docs: HashMap<usize, HashSet<usize>> = HashMap::new();

    for m_line in rel_reader.lines() {
        if let Ok(line) = m_line {
            let parts: Vec<String> = line.split_whitespace().map(|s| String::from(s)).collect();
            let query_id = parts.get(0).expect("Relevance line is not formatted correctly").parse::<usize>().unwrap();
            let doc_id = parts.get(2).expect("Relevance line is not formatted correctly")[5..].parse::<usize>().unwrap();
            let docs = relevant_docs.entry(query_id).or_insert(HashSet::new());
            docs.insert(doc_id.clone());
        }
    }

    relevant_docs
}
