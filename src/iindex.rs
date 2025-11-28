use std::collections::{HashMap, HashSet};

use crate::tokenizer::SimpleTokenizer;

#[derive(Default)]
pub struct InvertedIndex {
    core: HashMap<String, HashSet<usize>>,
    docs: HashMap<usize, String>,
    next_doc_id: usize,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a document to the inverted index by tokenizing it,
    /// and then expanding out the index, storing the original
    /// document in its untokenized form
    pub fn insert_document(&mut self, doc: &str) {
        let tokens = SimpleTokenizer::tokenize(doc);
        self.docs.insert(self.next_doc_id, doc.to_string());
        for token in tokens {
            let doc_list = self.core.entry(token).or_default();
            doc_list.insert(self.next_doc_id);
        }
        self.next_doc_id += 1;
    }

    /// Get a document via its id
    pub fn get_document(&self, id: usize) -> Option<&String> {
        self.docs.get(&id)
    }

    /// Perform a search "OR" on the index, returning the
    /// doc id "hits". This means that it will return all documents
    /// that match **at least one** token from the query (more lax).
    pub fn search_or(&self, query: &str) -> HashSet<usize> {
        let query_tokens = SimpleTokenizer::tokenize(query);
        let mut hits = HashSet::new();

        for token in query_tokens {
            if let Some(doc_ids) = self.core.get(&token) {
                hits.extend(doc_ids);
            }
        }

        hits
    }

    /// Perform a search "AND" on the index, returning the
    /// doc id "hits". This means that it will return all documents
    /// that match **all tokens** from the query (more conservative).
    pub fn search_and(&self, query: &str) -> HashSet<usize> {
        let query_tokens = SimpleTokenizer::tokenize(query);
        let mut hits: Option<HashSet<usize>> = None;

        for token in query_tokens {
            if let Some(doc_ids) = self.core.get(&token) {
                let doc_set: HashSet<usize> = doc_ids.iter().copied().collect();
                hits = Some(match hits {
                    None => doc_set,
                    Some(current) => current.intersection(&doc_set).copied().collect(),
                });
            } else {
                // token not found in any documents, so no results
                return HashSet::new();
            }
        }

        hits.unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_retrieve_document() {
        let mut index = InvertedIndex::new();
        index.insert_document("hello world");

        assert_eq!(index.get_document(0), Some(&"hello world".to_string()));
    }

    #[test]
    fn test_multiple_documents() {
        let mut index = InvertedIndex::new();
        index.insert_document("hello world");
        index.insert_document("foo bar");
        index.insert_document("hello foo");

        assert_eq!(index.get_document(0), Some(&"hello world".to_string()));
        assert_eq!(index.get_document(1), Some(&"foo bar".to_string()));
        assert_eq!(index.get_document(2), Some(&"hello foo".to_string()));
    }

    #[test]
    fn test_search_or_single_match() {
        let mut index = InvertedIndex::new();
        index.insert_document("hello world");
        index.insert_document("foo bar");

        let results = index.search_or("hello");
        assert_eq!(results, vec![0].into_iter().collect());
    }

    #[test]
    fn test_search_or_multiple_matches() {
        let mut index = InvertedIndex::new();
        index.insert_document("hello world");
        index.insert_document("hello foo");
        index.insert_document("bar baz");

        let results = index.search_or("hello");
        assert_eq!(results.len(), 2);
        assert!(results.contains(&0));
        assert!(results.contains(&1));
    }

    #[test]
    fn test_search_or_multiple_tokens() {
        let mut index = InvertedIndex::new();
        index.insert_document("hello world");
        index.insert_document("foo bar");
        index.insert_document("baz qux");

        let results = index.search_or("hello foo");
        assert_eq!(results.len(), 2);
        assert!(results.contains(&0));
        assert!(results.contains(&1));
    }

    #[test]
    fn test_search_or_no_matches() {
        let mut index = InvertedIndex::new();
        index.insert_document("hello world");

        let results = index.search_or("notfound");
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_and_all_tokens_present() {
        let mut index = InvertedIndex::new();
        index.insert_document("hello world");
        index.insert_document("hello foo world");
        index.insert_document("foo bar");

        let results = index.search_and("hello world");
        assert_eq!(results.len(), 2);
        assert!(results.contains(&0));
        assert!(results.contains(&1));
    }

    #[test]
    fn test_search_and_partial_match() {
        let mut index = InvertedIndex::new();
        index.insert_document("hello world");
        index.insert_document("hello foo");
        index.insert_document("world bar");

        let results = index.search_and("hello world");
        assert_eq!(results.len(), 1);
        assert!(results.contains(&0));
    }

    #[test]
    fn test_search_and_no_matches() {
        let mut index = InvertedIndex::new();
        index.insert_document("hello world");
        index.insert_document("foo bar");

        let results = index.search_and("hello notfound");
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_and_single_token() {
        let mut index = InvertedIndex::new();
        index.insert_document("hello world");
        index.insert_document("foo bar");

        let results = index.search_and("hello");
        assert_eq!(results.len(), 1);
        assert!(results.contains(&0));
    }

    #[test]
    fn test_case_insensitive_search() {
        let mut index = InvertedIndex::new();
        index.insert_document("Hello World");

        let results = index.search_or("hello");
        assert!(results.contains(&0));
    }

    #[test]
    fn test_punctuation_removed() {
        let mut index = InvertedIndex::new();
        index.insert_document("Hello, World!");

        let results = index.search_or("hello world");
        assert_eq!(results.len(), 1);
        assert!(results.contains(&0));
    }
}
