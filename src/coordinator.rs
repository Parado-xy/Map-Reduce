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
    /// The split function gets the file at `file_path`
    /// Then it splits it into chunks of `size = file_size / self.folds`
    /// It returns a `Vec<String>`, where each `String` is a processable chunk.
    pub fn split<P: AsRef<Path>>(self, file_path: P) -> Result<Vec<String>, Box<dyn Error>> {
        // Let's get a handle on the file.
        let mut file = File::open(file_path)?;
        // Get the file metadata.
        let metadata = file.metadata()?;
        // Get the length of the file in bytes.
        let bytes = metadata.len() as usize;
        // Calculate chunk size based on folds.
        let chunk_size = bytes / self.folds;
        // Get a buffer for reading.
        let mut reader = BufReader::new(file);

        let mut chunks = Vec::new();
        let mut buffer = vec![0; chunk_size];
        // We need this incase the chunk doesn't read a complete word.
        let mut leftover_word = String::new();
        
        // The loop runs till we reach the end of the file.
        loop {
            // Read some bytes to the buffer.
            // This moves the read head of the buffer forward
            let bytes_read = reader.read(&mut buffer)?;
            // If no bytes are read, it means we've reached the end of the file.
            if bytes_read == 0 {
                // if we have a leftover_word
                if !leftover_word.is_empty() {
                    // add it to the chunks vector.
                    chunks.push(leftover_word);
                }

                break; // End Of File
            }

            // Convert each chunk to a string.
            // We read from the begining of the buffer to the end of bytes-read because the buffer may have not been full, and we don't want to decode blank values
            let mut current_chunk = String::from_utf8_lossy(&buffer[..bytes_read]).to_string().to_lowercase();
            // Attach the leftover word to the beginning of the string.
            current_chunk.insert_str(0, &leftover_word);
            // clear the leftover_word string.
            leftover_word.clear();

            // Check if the last word is complete.
            let last_space_index = current_chunk.rfind(|c: char| c.is_ascii_whitespace()); // This finds the index of the last ascii whitespace. 

            match last_space_index {
                Some(idx) => {
                    // If the last word is incomplete and not the only word, save it for the next chunk
                    if idx < current_chunk.len() - 1 {
                        let (complete, incomplete) = current_chunk.split_at(idx + 1);
                        chunks.push(complete.to_string());
                        leftover_word.push_str(incomplete.trim_start());
                    } else {
                        // Entire chunk is complete
                        chunks.push(current_chunk);
                    }
                }
                None => {
                    // No spaces in the chunk. It's one long word. Append and continue.
                    leftover_word = current_chunk;
                }
            }
        }

        Ok(chunks)
    }

    /// This is the shuffle function. It will recieve every `mapped` file, and combine
    /// it in the `main_map`. It takes two arguments `main_map` and `recieved_map`
    /// They both are of type `HashMap<String, usize>`
    pub fn shuffle(self, main_map: &mut HashMap<String, usize>, recieved_map: HashMap<String, usize>) {

        for key in recieved_map.keys(){
            if let Some(count) = recieved_map.get(key){
                *main_map.entry(key.clone()).or_insert(0) += count; 
            }
        }
    }
}
