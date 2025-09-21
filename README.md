
### **Question Prompt**

You are asked to implement a simplified MapReduce framework in Rust. Your task is to build a program that:

1. **Splits input data** into fixed-size chunks.
2. **Maps** each chunk into intermediate key-value pairs using a user-defined `map` function.
3. **Shuffles** and groups intermediate pairs by key.
4. **Reduces** each group into a final result using a user-defined `reduce` function.

The concrete problem:

> You are given a collection of text files (e.g., `book1.txt`, `book2.txt`, `book3.txt`). Write a Rust program that uses a map-reduce pipeline to compute the **top 10 most frequent words across all files**.

---

### **Details / Requirements**

* **Map Phase:**

  * The `map` function should take a chunk of text and output key-value pairs of the form `(word, 1)`.

* **Shuffle Phase:**

  * Group all intermediate pairs so that each unique word is associated with a list of counts.

* **Reduce Phase:**

  * The `reduce` function should sum the counts for each word.

* **Final Output:**

  * Print the top 10 most frequent words with their counts.

---

### **Rust-Specific Expectations**

* **Concurrency:**

  * Use Rust threads (or a threadpool like `rayon`) to parallelize the `map` phase across chunks.
  * Manage synchronization safely with channels (`std::sync::mpsc`) or shared state (`Arc<Mutex<_>>`).

* **Ownership & Borrowing:**

  * Handle chunking and passing data to workers without unnecessary cloning.

* **Error Handling:**

  * Handle cases where a file cannot be read.

---

### **What’s being tested?**

* Understanding of the **MapReduce abstraction** (map, shuffle, reduce).
* Ability to **parallelize workloads** and coordinate between threads.
* Use of **Rust concurrency primitives** safely (ownership, borrowing, synchronization).
* Structuring code with **traits and generics**, allowing different `map` and `reduce` functions to be plugged in.

---

### **Possible Extension (for ambitious students)**

* Make the framework generic: allow plugging in different `map`/`reduce` functions, not just word count.
* Distribute across multiple processes using TCP sockets or message passing.
