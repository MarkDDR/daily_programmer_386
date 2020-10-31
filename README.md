# Partition Counts

This program calculates the number of unique ways to partition a number into different sums. For example, the number 5 can be partitioned in the following ways:

```
5
4 + 1
3 + 1 + 1
3 + 2
2 + 2 + 1
2 + 1 + 1 + 1
1 + 1 + 1 + 1 + 1
```

Here we see there are a total of 7 ways to partition the number 5, so we say `p(5) = 7`.

More information about the problem and algorithms can be found on the [Daily Programmer #386](https://old.reddit.com/r/dailyprogrammer/comments/jfcuz5/20201021_challenge_386_intermediate_partition/) and [Mathologer's video](https://www.youtube.com/watch?v=iJ8pnCO0nTY) on the topic.

This program has both a single threaded and a multithreaded mode, both of which aim to be as fast as possible.

# Introducing Multithreading to the Algorithm
The algorithm at first appears to all need to be done linearly, since `p(n)` is a recursive sequence defined as `p(n) = p(n-1) + p(n-2) - p(n-5) ...`, importantly needing the previous term to calculate the next term.
But what we can do to speed it up is calculate part of the `p(n)` with the terms we know while we wait for its closer dependent terms (like `p(n-1)`, `p(n-2)`, etc.) being calculated in other threads to "resolve".
We can do this in a lock free manner too by using atomic integers: We define a table with enough space to store all the values we want without having to reallocate, and then only allow threads to look at a slice of "resolved" values that will never change (i.e. `0..last_term_calculated`).
Then each thread gets a special unique ticket that allows it to set the value of the `n`th term it is trying to resolve.
Once it uses that ticket to set the value, other threads will be free to look at it (i.e. `0..n`), and since a ticket cannot be used more than once, no data races can occur since the algorithm relies on `n-1` to be resolved before `n` can get resolved.

# Running
Besides rust and cargo, you will also need the GMP library installed (which is what this program uses for bignums, via [`rug`](https://crates.io/crates/rug)). Afterwards, you can build and run with these 2 commands in the project root.

```
$ cargo build --release
$ cargo run --release
```

If you want to switch between the multithreaded and single threaded versions of the code, you will need to manually comment/uncomment the relevant function calls in the main function in `main.rs`

# Speed
My testing machine has a relatively old 4 core cpu at 3.6 GHz.

Single Threaded: `p(666,666)` takes about 65 seconds

Multithreaded: `p(666,666)` takes about 18 seconds

## License
[MIT](https://choosealicense.com/licenses/mit/)