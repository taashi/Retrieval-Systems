use select::predicate::*;
use select::document::Document;
use regex::Regex;

use std::fs::File;
use std::io::BufReader;

/// Reads in an html document from `in_file`, removes html tags, optionally does
/// case folding and punctuation stripping, writing result file to `out_file`.
pub fn parse(in_file: File, case_fold: bool, strip_punc: bool) -> String{
    let doc = Document::from_read(BufReader::new(in_file)).unwrap();
    let html_stripped = strip_html(doc);
    let punct_case_folded = format(html_stripped, case_fold, strip_punc);
    let res = remove_trailing_numbers(punct_case_folded);
    res
}

/// Assumption made for this project: all documents have exactly one pre tag
/// and all text is within that tag.
fn strip_html(doc: Document) -> String {
    if doc.find(Name("pre")).count() != 1 {
        panic!("Expected HTML to have exectly one pre tag, it does not");
    }
    doc.find(Name("pre")).nth(0).unwrap().text()
}

/// Optionally removes punctuation and converts string to lowercase
/// depending on provided params.
fn format(line: String, case_fold: bool, strip_punc: bool) -> String {
    let re_punct_to_space = Regex::new(r"[-,]").unwrap();
    let re_punct_remove = Regex::new(r"[\W&&\S]").unwrap();
    let re_whitespace = Regex::new(r"\s+").unwrap();
    let mut s = line.clone();

    if strip_punc {
        s = String::from(re_punct_to_space.replace_all(s.as_str(), " "));
        s = String::from(re_punct_remove.replace_all(s.as_str(), ""));
        s = String::from(re_whitespace.replace_all(s.as_str(), " "));
    }

    if case_fold {
        s = s.to_lowercase();
    }
    s
}

/// Removes the trailing numbers at the end of any document
pub fn remove_trailing_numbers(contents: String) -> String {
    let mut term_iter = contents.split_whitespace().rev();
    let mut n = term_iter.next().expect("Document is empty");
    let mut still_number = n.parse::<usize>().is_ok();
    while still_number {
        if let Some(next) = term_iter.next() {
            n = next;
            still_number = n.parse::<usize>().is_ok();
        }
    }
    let mut ret = String::new();
    for term in term_iter.rev() {
        ret.push(' ');
        ret.push_str(term);
    }
    ret.push(' ');
    ret.push_str(n);
    ret
}
