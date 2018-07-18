use std::collections::{HashMap, HashSet};

use Index;
use Retriever;

pub struct BM25 {
    index: Index,
    avdl: f64,
    // Term to number of documents containing that term
    doc_freq: HashMap<String, usize>,
    // Document id to length cache
    doc_len: HashMap<String, usize>,
    corpus_size: f64,
}

impl BM25 {
    pub fn new(index: Index) -> BM25 {
        let mut doc_lengths = HashMap::new();
        let mut doc_f = HashMap::new();
        let mut doc_ids = HashSet::new();

        for(term, i_list) in index.iter() {
            for (doc, freq) in i_list.iter() {
                let len = doc_lengths.entry(doc.clone()).or_insert(0);
                *len += freq;
                let f = doc_f.entry(term.clone()).or_insert(0);
                *f += 1;
                doc_ids.insert(doc.clone());
            }
        }

        let total_len_all: usize = doc_lengths.values().sum();

        BM25 {
            avdl: (total_len_all as f64) / (doc_lengths.len() as f64),
            doc_freq: doc_f,
            doc_len: doc_lengths,
            index: index,
            corpus_size: doc_ids.len() as f64,
        }
    }
}

impl Retriever for BM25 {
    fn rank(&mut self, query: Vec<String>) -> Vec<(String, f64)> {
        let mut documents = HashSet::new();
        for qt in query.iter() {
            if let Some(il) = self.index.get(qt) {
                for doc in il.keys() {
                    documents.insert(doc.clone());
                }
            }
        }


        let mut results: Vec<(String, f64)> = Vec::new();
        for doc in documents.iter() {
            let mut sum = 0.0;
            for term in query.iter() {
                let ni: f64 = *self.doc_freq.get(term).unwrap_or(&0) as f64;
                let n = self.corpus_size;

                let term_il = match self.index.get(term) {
                    Some(e) => e.clone(),
                    None    => HashMap::new(),
                };

                let fi: f64 = *term_il.get(doc).unwrap_or(&0) as f64;
                let ni_o_n_ni: f64 = (ni + 0.5) / (n - ni + 0.5);
                let k1_fi: f64 = (1.2 + 1.0) * fi;
                let dl: f64 = *self.doc_len.get(doc).unwrap() as f64;
                let k: f64 = 1.2 * ((1.0 - 0.75) + (0.75 * (dl / self.avdl)));
                let matching_query_terms: Vec<&String> = query.iter()
                    .filter(|s| s.to_string() == term.to_string())
                    .collect();
                let qfi: usize = matching_query_terms.len();

                let t_1: f64 = 1.0 / ni_o_n_ni;
                let t_2: f64 = k1_fi / (k + fi);
                let t_3: f64 = (101 * qfi) as f64 / (100 + qfi) as f64;

                sum += t_1.log2() * t_2 * t_3;
            }
            results.push((doc.clone(), sum));
        }
        results.sort_by(|a, b| (b.1).partial_cmp(&a.1).unwrap());
        results
    }
}
