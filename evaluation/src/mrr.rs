use std::collections::{HashSet, HashMap};

pub struct Mrr { }

impl Mrr {
    /// Calculates the Mean Recriprocal Rank of all queries provided.
    pub fn calc(results: &HashMap<usize, Vec<usize>>, relevance_info: &HashMap<usize, HashSet<usize>>) -> f64 {
        let mut count = 0.0;
        let mut sum   = 0.0;
        for (query, docs) in relevance_info.iter() {
            sum += Mrr::calc_recrip_rank(results.get(query).unwrap(), docs);
            count += 1.0;
        }

        sum / count
    }

    /// Given a list of retrieved documents and the set of relevant ones,
    /// returns the recriprocal of the rank of the first relevant document.
    pub fn calc_recrip_rank(retrieved_docs: &Vec<usize>, relevant_docs: &HashSet<usize>) -> f64 {
        let mut rank = 1.0;
        for doc in retrieved_docs.iter() {
            if relevant_docs.contains(doc) {
                return 1.0 / rank;
            }
            rank += 1.0;
        }

        0.0
    }
}
