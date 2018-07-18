use std::collections::{HashSet, HashMap};

pub struct Map { }

impl Map {
    /// Calculates the Mean Average Precision of all queries provided.
    pub fn calc(results: &HashMap<usize, Vec<usize>>, relevance_info: &HashMap<usize, HashSet<usize>>) -> f64 {
        let mut count = 0.0;
        let mut sum   = 0.0;
        for (query, docs) in relevance_info.iter() {
            sum += Map::calc_average_precision(results.get(query).unwrap(), docs);
            count += 1.0;
        }

        sum / count
    }

    /// Given a list of retrieved documents and the set of relevant
    /// ones, returns theAverage Precision across the query.
    pub fn calc_average_precision(retrieved_docs: &Vec<usize>, relevant_docs: &HashSet<usize>) -> f64 {
        let mut relevant  = 0.0;
        let mut retrieved = 0.0;
        let mut sum       = 0.0;

        for doc in retrieved_docs.iter() {
            if relevant_docs.contains(doc) {
                relevant += 1.0;
            }
            retrieved += 1.0;
            sum += relevant / retrieved;
        }

        sum / (retrieved_docs.len() as f64)
    }
}
