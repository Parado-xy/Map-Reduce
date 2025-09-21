use std::{
    collections::HashMap, error::Error, sync::mpsc::{channel, Sender}
};

/// This function will use a mpsc channel.
/// It is a produser function and yields a mapping of words to their count.
/// `HashMap<String, usize>`
pub fn map(chunk: String, sender: Sender<HashMap<String, usize>>) -> Result<(), Box<dyn Error>> {
    // Instantiate a word count map. 
    let mut word_count_map: HashMap<String, usize> = HashMap::new();

    // Split the long string on whitespace to get individual words. 
    for word in chunk.split_whitespace(){
        // Get a mutable refrence to the count of the word. 
        if let Some(count) = word_count_map.get_mut(word){
            // If the word is present, increment the count; 
            *count += 1; 
        }else{
            // If the word is not present, add it to the word_count_map
            word_count_map.insert(word.to_owned(), 1); 
        }
    } 

    // After processig this chunk, send it to the reciever. 
    sender.send(word_count_map)?;

    Ok(())
}
