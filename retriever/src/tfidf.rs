use std::collections::{HashMap, HashSet};

use Index;
use Retriever;

pub struct TfIdf {
    index: Index,
    // Term to number of documents containing that term
    doc_freq: HashMap<String, usize>,
    // Document id to length cache
    doc_len: HashMap<String, usize>,
}

impl TfIdf {
    pub fn new(index: Index) -> TfIdf {
        let mut doc_lengths = HashMap::new();
        let mut doc_f = HashMap::new();

        for(term, i_list) in index.iter() {
            for (doc, freq) in i_list.iter() {
                let len = doc_lengths.entry(doc.clone()).or_insert(0);
                *len += freq;
                let f = doc_f.entry(term.clone()).or_insert(0);
                *f += 1;
            }
        }

        TfIdf {
            doc_freq: doc_f,
            doc_len: doc_lengths,
            index: index,
        }
    }
}

impl Retriever for TfIdf {
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
            let mut tfidf_sum = 0.0;
            for qt in query.iter() {
                let term_occurances = self.index.get(qt).unwrap_or(&empty_il).get(doc).unwrap_or(&0);
                let tf = (*term_occurances as f64) / (*self.doc_len.get(doc).expect("Document missing from length cache") as f64);
                let idf = match self.doc_freq.get(qt) {
                    Some(df) => (self.doc_len.keys().len() as f64 / (*df as f64)).log2(),
                    None => 0.0,
                };
                tfidf_sum += tf * idf;
            }

            results.push((doc.clone(), tfidf_sum));
        }
        results.sort_by(|a, b| (b.1).partial_cmp(&a.1).unwrap());
        results
    }
}
