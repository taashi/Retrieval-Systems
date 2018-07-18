use std::collections::{HashSet, HashMap};

pub struct Pak { }

impl Pak {
    /// Calculates the P@K value for each of the queries provided.
    /// Returns a list of (query_id, P@K) tuples
    pub fn calc(k: usize, results: &HashMap<usize, Vec<usize>>, relevance_info: &HashMap<usize, HashSet<usize>>) -> Vec<(usize, f64)> {
        let mut ret: Vec<(usize, f64)> = Vec::new();
        for (query, docs) in relevance_info.iter() {
            ret.push((*query, Pak::calc_pk(k, results.get(query).unwrap(), docs)));
        }
        ret.sort_by_key(|tup| tup.0);
        ret
    }

    /// Given k, a list of retrieved documents and the set of relevant
    /// ones, returns the P@K value for the query.
    pub fn calc_pk(k: usize, retrieved_docs: &Vec<usize>, relevant_docs: &HashSet<usize>) -> f64 {
        let mut relevant  = 0.0;
        let mut retrieved = 0.0;

        for doc in retrieved_docs.iter() {
            if relevant_docs.contains(doc) {
                relevant += 1.0;
            }
            retrieved += 1.0;
            if retrieved as usize == k {
                return relevant / retrieved
            }
        }

        relevant / retrieved
    }
}
