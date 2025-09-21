use std::{collections::HashMap};

pub fn reduce(mapped: HashMap<String, usize>, most_freq: usize) -> Vec<(String, usize)>{

    let mut mapped_vec: Vec<(&String, &usize)> = mapped.iter().collect(); 
    // Sort in descending order.
    mapped_vec.sort_by(|a, b| b.1.cmp(a.1));

    mapped_vec[..most_freq.min(mapped_vec.len())]
        .iter()
        .map(|(k, v)| (k.to_string(), **v))
        .collect()
}