// https://old.reddit.com/r/dailyprogrammer/comments/jfcuz5/20201021_challenge_386_intermediate_partition/
// Partition Counts

pub mod bigint;
pub mod multithreaded;
pub mod util;

use bigint::BigInt;
use multithreaded::*;
use util::*;

use std::sync::Arc;
use threadpool::ThreadPool;

pub fn calc_partition_count(n: usize) -> BigInt {
    let index_subtractions = generate_index_subtractions(n);
    let mut index_subtraction_take_count = 2;

    let mut partition_count_table = vec![BigInt::from(1), BigInt::from(1)];

    for index in 2..=n {
        if index_subtractions
            .get(index_subtraction_take_count)
            .map(|x| x.0)
            == Some(index)
        {
            index_subtraction_take_count += 1;
        }
        let index_sub_slice = &index_subtractions[0..index_subtraction_take_count];

        let mut partial_sum = BigInt::from(0);
        for (sub_amt, add_or_sub) in index_sub_slice.iter().rev() {
            let int = &partition_count_table[index - sub_amt];
            match add_or_sub {
                AddOrSub::Add => partial_sum += int,
                AddOrSub::Sub => partial_sum -= int,
            }
        }

        partition_count_table.push(partial_sum);
    }

    partition_count_table[n].clone()
}

pub fn calc_partition_count_parallel(n: usize, num_threads: usize) -> BigInt {
    let index_subtractions = Arc::new(generate_index_subtractions(n));
    let partition_count_table = Arc::new(ParallelTable::new(n));

    // let num_threads = num_cpus::get();
    let pool = ThreadPool::new(num_threads);

    let table_mut_tickets = generate_tickets(partition_count_table.clone()).unwrap();

    for ticket in table_mut_tickets {
        let partition_count_table = partition_count_table.clone();
        let index_subtractions = index_subtractions.clone();

        pool.execute(move || {
            let mut index_subtraction_take_count = 2;
            for (i, (sub_amt, _)) in index_subtractions.iter().enumerate().skip(2) {
                if ticket.get_index() >= *sub_amt {
                    index_subtraction_take_count = i + 1;
                } else {
                    break;
                }
            }

            let index_sub_slice = &index_subtractions[0..index_subtraction_take_count];
            let part_valid = partition_count_table.get_all_valid();
            let mut partial_sum = BigInt::from(0);

            for (sub_amount, add_or_sub) in index_sub_slice.iter().rev() {
                let get_index = ticket.get_index() - sub_amount;
                let int: &BigInt = match part_valid.get(get_index) {
                    Some(int) => int.as_ref().unwrap(),
                    None => loop {
                        match partition_count_table.get(get_index) {
                            Some(int) => break int,
                            None => {}
                        }
                    },
                };
                match add_or_sub {
                    AddOrSub::Add => partial_sum += int,
                    AddOrSub::Sub => partial_sum -= int,
                }
            }
            ticket.set(partial_sum);
        });
    }

    pool.join();

    partition_count_table
        .get(n)
        .as_ref()
        .expect("didn't calculate up to n somehow")
        .clone()
}
