use std::{collections::HashMap, error::Error, sync::mpsc, thread, time::Duration};
mod coordinator;
mod map;
mod reduce;

fn main() -> Result<(), Box<dyn Error>> {
    let folds = 5usize;
    let job_coordinator = coordinator::Coordinator { folds };
    let file_path = "test_files/known_counts.txt";
    let chunks = job_coordinator.split(file_path)?;
     

    let (sender, receiver) = mpsc::channel();

    let mut handles = vec![];
    for chunk in chunks {
        let chunk_sender = sender.clone();
        let handle = thread::spawn(move || {
            if let Err(e) = map::map(chunk, chunk_sender) {
                eprintln!("Error in map function: {:?}", e);
            }
        });
        handles.push(handle);
    }

    let mut main_map = HashMap::new();

    for _ in 0..folds {
        // Process received data in the main thread
        match receiver.recv_timeout(Duration::from_secs(10)) {
            Ok(recieved_map) => {
                job_coordinator.shuffle(&mut main_map, recieved_map);
            }
            Err(e) => {
                eprintln!("Timeout for mapping_process: {e}");
            }
        }
    }

    // Now reduce to the to most frequent values.
    let result = reduce::reduce(main_map, 10); 

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("{result:?}"); 

    Ok(())
}
