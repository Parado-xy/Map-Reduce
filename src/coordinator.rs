// Here in lies the coordinator for the map reduce process accross threads.

use std::collections::HashMap;
use std::error::Error;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

// Declare the coordinator struct.
#[derive(Clone, Copy)]
pub struct Coordinator {
    /// The number of ways to split the file.
    pub folds: usize,
}
impl Coordinator {
    /// Splits a file into roughly `self.folds` chunks.
    /// Each chunk ends on a word boundary, so words aren't cut in half.
    pub fn split<P: AsRef<Path>>(self, file_path: P) -> Result<Vec<String>, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let metadata = file.metadata()?;
        let total_bytes = metadata.len() as usize;

        // Use ceiling division so we don't lose bytes on the last chunk
        let chunk_size = (total_bytes + self.folds - 1) / self.folds;

        let mut reader = BufReader::new(file);
        let mut chunks = Vec::new();

        let mut buffer = vec![0; chunk_size];
        let mut leftover = String::new();

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                if !leftover.is_empty() {
                    chunks.push(leftover);
                }
                break;
            }

            // Convert the bytes to UTF-8 (lossy handles broken sequences safely)
            let mut current_chunk = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

            // Prepend leftover word from last iteration
            if !leftover.is_empty() {
                current_chunk.insert_str(0, &leftover);
                leftover.clear();
            }

            // Find the last whitespace to avoid splitting in the middle of a word
            if let Some(idx) = current_chunk.rfind(|c: char| c.is_ascii_whitespace()) {
                let (complete, incomplete) = current_chunk.split_at(idx + 1);
                chunks.push(complete.to_string());
                leftover.push_str(incomplete.trim_start());
            } else {
                // No whitespace at all: one long word continues to next chunk
                leftover.push_str(&current_chunk);
            }
        }

        Ok(chunks)
    }

    /// Combines word counts from `recieved_map` into `main_map`.
    pub fn shuffle(
        self,
        main_map: &mut HashMap<String, usize>,
        recieved_map: HashMap<String, usize>,
    ) {
        for (key, count) in recieved_map {
            *main_map.entry(key).or_insert(0) += count;
        }
    }
}
