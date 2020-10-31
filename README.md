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

More information can be found on the [Daily Programmer #386](https://old.reddit.com/r/dailyprogrammer/comments/jfcuz5/20201021_challenge_386_intermediate_partition/) and [Mathologer's video](https://www.youtube.com/watch?v=iJ8pnCO0nTY) on the topic.

This program has both a single threaded and a multithreaded mode, which aim to be as fast as possible.

# Running
Besides rust and cargo, you will also need the GMP library installed (which is what this program uses for bignums, via [`rug`](https://crates.io/crates/rug)). Afterwards, you can build and run with these 2 commands.
```
$ cargo build --release
$ cargo run --release
```

# Speed
My testing machine has a relatively old 4 core cpu at 3.6 GHz.

Single Threaded: `p(666,666)` takes about 105 seconds

Multithreaded: `p(666,666)` takes about 18 seconds

## License
[MIT](https://choosealicense.com/licenses/mit/)