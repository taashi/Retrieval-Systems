use std::collections::{HashMap, HashSet};

use Index;
use Retriever;

pub const LAMBDA: f64 = 0.35;

pub struct QueryLikelihood {
    index: Index,
    // Document id to length cache
    doc_len: HashMap<String, usize>,
    // Total length of the corpus
    corpus_len: usize,
}

impl QueryLikelihood {
    pub fn new(index: Index) -> QueryLikelihood {
        let mut doc_lengths = HashMap::new();
        let mut corpus_len = 0;

        for(_, i_list) in index.iter() {
            for (doc, freq) in i_list.iter() {
                let len = doc_lengths.entry(doc.clone()).or_insert(0);
                *len += freq;
                corpus_len += freq;
            }
        }

        QueryLikelihood {
            doc_len: doc_lengths,
            corpus_len: corpus_len,
            index: index,
        }
    }
}

impl Retriever for QueryLikelihood {
    fn rank(&mut self, query: Vec<String>) -> Vec<(String, f64)> {
        let mut documents = HashSet::new();
        for qt in query.iter() {
            if let Some(il) = self.index.get(qt) {
                for doc in il.keys() {
                    documents.insert(doc.clone());
                }
            }
        }

        let empty_il = HashMap::new();

        let mut results: Vec<(String, f64)> = Vec::new();
        for doc in documents.iter() {
            let mut ql_sum = 0.0;
            for qt in query.iter() {
                let term_occurances: f64 = *self.index.get(qt).unwrap_or(&empty_il).get(doc).unwrap_or(&0) as f64;
                let doc_len: f64 = *self.doc_len.get(doc).expect("Document missing from length cache") as f64;
                let t1: f64 = (1.0 - LAMBDA) * (term_occurances / doc_len);
                let mut corpus_occurances: f64 = 0.0;
                if let Some(il) = self.index.get(qt) {
                    for (_, o) in il.iter() {
                        corpus_occurances += *o as f64;
                    }
                };
                let t2 = LAMBDA * corpus_occurances / (self.corpus_len as f64);

                if t1 + t2 != 0.0 {
                    ql_sum += (t1 + t2).log2();
                }
            }
            results.push((doc.clone(), ql_sum));
        }
        results.sort_by(|a, b| (b.1).partial_cmp(&a.1).unwrap());
        results
    }
}
