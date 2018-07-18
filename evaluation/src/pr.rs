use std::collections::{HashSet, HashMap};

pub struct PrecisionAndRecall { }

impl PrecisionAndRecall {
    /// Calculates the Precision and Recall of all queries provided at all ranks.
    /// Returns a list of tuples (query_id, (rank, precision, recall))
    /// in order by query_id and rank
    pub fn calc(results: &HashMap<usize, Vec<usize>>, relevance_info: &HashMap<usize, HashSet<usize>>) -> Vec<(usize, usize, f64, f64)> {
        let mut ret: Vec<(usize, usize, f64, f64)> = Vec::new();
        for (query, docs) in relevance_info.iter() {
            for (rank, precision, recall) in PrecisionAndRecall::calc_one_query(results.get(query).unwrap(), docs) {
                ret.push((*query, rank, precision, recall));
            }

        }
        ret.sort_by(|a, b| (a.0 * 100 + a.1).cmp(&(b.0 * 100 + b.1)));
        ret
    }

    /// Given a list of retrieved documents and the set of relevant ones,
    /// returns the Precision and Recall at each rank formatted in a list of tuple (rank, precision, recall)
    pub fn calc_one_query(retrieved_docs: &Vec<usize>, relevant_docs: &HashSet<usize>) -> Vec<(usize, f64, f64)> {
        let mut ret = Vec::new();

        let num_relevant_docs = relevant_docs.len() as f64;
        let mut rank      = 0;
        let mut relevant  = 0.0;
        let mut retrieved = 0.0;

        for doc in retrieved_docs.iter() {
            if relevant_docs.contains(doc) {
                relevant += 1.0;
            }
            retrieved += 1.0;
            rank += 1;

            let precision = relevant / retrieved;
            let recall = relevant / num_relevant_docs;
            ret.push((rank, precision, recall));
        }

        ret.sort_by_key(|tup| tup.0);
        ret
    }
}
