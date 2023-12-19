# 👨🏼‍💻 Exercises
These exercises aren't necessarily about getting the best performance, but about trying combinations
of the concepts introduced in the module. It can still be interesting to time and compare the different strategies.

## Parallel Sums and Histograms
For this series of exercises we'll look at using some parallelization mechanisms
for summing a list of numbers or creating a histogram of the number of occurences
of different integer values.

First we'll create a new project which can contain all of the exercises.
Create a new Rust project using ```cargo new parallel_sums_and_histograms```.

=== "Cargo.toml"

    ```rust
    [dependencies]
    rayon = "*"
    rand = "*"
    ```

=== "main.rs"

    ```rust
    use rand::{Rng, rngs::ThreadRng};
    use std::sync::{atomic::AtomicU64, Mutex, MutexGuard, mpsc::channel};
    use rayon::prelude::*;

    const BIN_COUNT: usize = 16;
    const THREAD_COUNT: usize = 4;

    fn main() {
        let mut rng: ThreadRng = rand::thread_rng();
        let chunk_size: usize = 4;
        let data_count: usize = 24;
        let data: Vec<u32> = (0..data_count).into_iter().map(|_| rng.gen_range(0..BIN_COUNT) as u32 ).collect();

        println!("Correct sum is: {}", data.iter().sum::<u32>());
        parallel_sum(&data);
        parallel_sum_with_atomic(&data, chunk_size);
        parallel_histogram_with_atomics(&data, chunk_size);
        parallel_histogram_with_lock(&data, chunk_size);
        parallel_histogram_with_threads_and_channels(&data);
    }
    ```

### m2::e0 - Parallel Sum
Create the parallel sum function. It should take in the data vector and sum the numbers using Rayon. Do it as
simply as possible.

??? note "Hints"

    .par_iter().sum() is almost all you need.

### m2::e1 - Parallel Sum with Atomic
Now, create and fill out the parallel sum with atomic function. Instead of using the .sum() function, try and create
an atomic variable outside of the iterator for summing the data. You can use either .par_iter() or .par_chunks().

??? note "Hints"

    To solve this in very few lines, use .par_chunks(chunk_size), .for_each(), AtomicU64 and .fetch_add() with
    a relaxed ordering.

Why might you use .par_chunks instead()?

??? note "Answer"
    Summing as many of the values as possible locally mitigates the contention for the atomic sum variable.

### m2::e2 - Parallel Histogram with Atomics
For the parallel histogram with atomics function you should now create an array of atomics instead. For each
value add to the count for the bin.

??? note "Hints - Atomic Array"

    You can create a statically sized array of atomics using the following snippet -

    === "Rust"

        ```rust
        let e2: [AtomicU64; BIN_COUNT] = [0_u64; BIN_COUNT].map(|x| AtomicU64::new(x));
        ```

??? note "Hints - Contention"

    If you want to mitigate contention in the atomic array, make a chunk local array to
    compute a local histogram before adding the other elements.

    You can also lower contention by checking whether you have a 0 in the histogram
    element you are about to atomically add.

### m2::e3 - Parallel Histogram with Locks
Now instead of each thread accessing an individual element at a time, try using an array of bin values with the
entire array wrapped in a Mutex (lock). Why not use a RwLock instead of a Mutex?

### m2::e4 - Parallel Histogram with Channels
This is where things get a little nuts... In this exercise a number of threads will go presort the data into the
bins by sending them to corresponding channels. Then a thread per bin will receive the values and count how many
it has received. Once all of the senders have been dropped it will send its count and index through a channel to
the main thread which will accumulate the received values into the final histogram.

* PARTITION INTO CHUNKS - Partition the data vector into ```thread_count``` number of chunks using slices,
```&[u32]```, and ```.split_at()```.
* SORTING PAIRS - Setup a BIN_COUNT number of sender/receiver pairs using ```mpsc::channels```.
* ACCUMULATION PAIR - Setup a single sender/receiver pair, also using ```mpsc::channels```.
* SORTING THREADS - Give the slices to ```thread_count``` number of threads (use ```thread::spawn```).
Each thread should be given BIN_COUNT sender copies of the sorting pairs. Meaning if thread A sees a 5 it
should send that 5 through sender 5. Once the thread has been through all values it should terminate and drop
its held senders.
* COUNTING THREADS - Setup BIN_COUNT number of threads each taking a single sorting receiver. It will also take a
clone of the accumulation pair sender. It will block on the sorting receiver and count all the incoming
values until it receives a ```Err(_)``` value. Once it does so, it will send its accumulated count through its
clone of the accumulation sender and then terminate.
* MASTER ACCUMULATOR - The main thread will block on the accumulation receiver and take all incoming values,
I suggest ```(usize, u64)```, and use them for constructing a histogram. Once it receives an ```Err(_)```, meaning
all senders have been dropped, it should print its histogram.
