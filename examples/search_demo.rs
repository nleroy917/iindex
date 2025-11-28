use std::io::{self, Write};
use iindex::iindex::InvertedIndex;

fn main() {
    println!("=== Inverted Index Demo ===\n");

    // Create and populate the index
    let mut index = InvertedIndex::new();

    // Read sample documents
    let sample_docs = [
        "The quick brown fox jumps over the lazy dog",
        "A journey of a thousand miles begins with a single step",
        "To be or not to be, that is the question",
        "All that glitters is not gold",
        "The early bird catches the worm"
    ];

    println!("Indexing {} documents...\n", sample_docs.len());
    for (i, doc) in sample_docs.iter().enumerate() {
        index.insert_document(doc);
        println!("  Doc {}: {}", i, doc);
    }

    // Interactive search loop
    loop {
        println!("\n--- Search Menu ---");
        println!("1. Search (OR - any token matches)");
        println!("2. Search (AND - all tokens match)");
        println!("3. Show document by ID");
        println!("4. Exit");
        print!("\nChoose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => search_or(&index),
            "2" => search_and(&index),
            "3" => show_document(&index),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn search_or(index: &InvertedIndex) {
    print!("Enter search query: ");
    io::stdout().flush().unwrap();

    let mut query = String::new();
    io::stdin().read_line(&mut query).unwrap();
    let query = query.trim();

    let results = index.search_or(query);

    if results.is_empty() {
        println!("No documents found matching '{}'", query);
    } else {
        println!("\nFound {} documents matching '{}' (OR search):", results.len(), query);
        let mut sorted_results: Vec<_> = results.iter().collect();
        sorted_results.sort();
        for doc_id in sorted_results {
            if let Some(doc) = index.get_document(*doc_id) {
                println!("  [{}] {}", doc_id, doc);
            }
        }
    }
}

fn search_and(index: &InvertedIndex) {
    print!("Enter search query: ");
    io::stdout().flush().unwrap();

    let mut query = String::new();
    io::stdin().read_line(&mut query).unwrap();
    let query = query.trim();

    let results = index.search_and(query);

    if results.is_empty() {
        println!("No documents found matching all tokens in '{}'", query);
    } else {
        println!("\nFound {} documents matching all tokens in '{}' (AND search):", results.len(), query);
        let mut sorted_results: Vec<_> = results.iter().collect();
        sorted_results.sort();
        for doc_id in sorted_results {
            if let Some(doc) = index.get_document(*doc_id) {
                println!("  [{}] {}", doc_id, doc);
            }
        }
    }
}

fn show_document(index: &InvertedIndex) {
    print!("Enter document ID: ");
    io::stdout().flush().unwrap();

    let mut id_str = String::new();
    io::stdin().read_line(&mut id_str).unwrap();

    match id_str.trim().parse::<usize>() {
        Ok(id) => {
            if let Some(doc) = index.get_document(id) {
                println!("\nDocument {}:\n{}", id, doc);
            } else {
                println!("Document {} not found", id);
            }
        }
        Err(_) => println!("Invalid ID format"),
    }
}