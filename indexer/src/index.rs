use serde_json;

use std::fs::File;
use std::collections::HashMap;
use std::io::Write;

/// Indexer keeps state for all documents during the indexing process
pub struct Indexer {
    /// A unigram index is a mapping from Term to Inverted List
    unigram_i: HashMap<String, IList>,
}

/// An Inverted List is a mapping from Document to term frequency
type IList = HashMap<String, usize>;

impl Indexer {
    /// Creates a new Indexer with an empty unigram index
    pub fn new() -> Indexer {
        Indexer {
            unigram_i: HashMap::new(),
        }
    }

    /// Add the given line to the index under the doc `doc_id`.
    /// Terms in `line` should be whitespace separated.
    pub fn index(&mut self, line: String, doc_id: String) {
        for word_ref in line.split_whitespace() {
            let word = String::from(word_ref);
            self.update_unigram_i(word.clone(), &doc_id);
        }
    }

    /// Updates the unigram index, adding an occurance of `term` to
    /// document `doc_id`
    fn update_unigram_i(&mut self, term: String, doc_id: &String) {
        let il = self.unigram_i.entry(term).or_insert(new_il(doc_id));
        let current = il.entry(doc_id.clone()).or_insert(0);
        *current += 1;
    }

    /// Writes the unigram index to the file `index_f`
    pub fn write_un_index(&mut self, mut index_f: File) {
        index_f.write(serde_json::to_vec(&self.unigram_i).unwrap().as_slice()).unwrap();
    }
}

/// Creates a new inverted list with one document, `doc_id`, given a tf of zero
fn new_il(doc_id: &String) -> IList {
    let mut new_il = HashMap::new();
    new_il.insert(doc_id.clone(), 0);
    new_il
}
