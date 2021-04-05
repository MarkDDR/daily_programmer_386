// https://old.reddit.com/r/dailyprogrammer/comments/jfcuz5/20201021_challenge_386_intermediate_partition/
// Partition Counts

pub mod bigint;
pub mod multithreaded;
pub mod sync;
pub mod util;

use bigint::BigInt;
use multithreaded::*;
use sync::*;
use util::*;

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

    let mut sender_pool: Vec<mpsc::Sender<ParallelTableMutIndexTicket>> = vec![];
    let mut thread_handles = vec![];
    for _ in 0..num_threads {
        let (sender, receiver) = mpsc::channel();
        sender_pool.push(sender);
        let partition_count_table = partition_count_table.clone();
        let index_subtractions = index_subtractions.clone();

        let handle = thread::spawn(move || {
            while let Ok(ticket) = receiver.recv() {
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
                                None => hint::spin_loop(),
                            }
                        },
                    };
                    match add_or_sub {
                        AddOrSub::Add => partial_sum += int,
                        AddOrSub::Sub => partial_sum -= int,
                    }
                }
                ticket.set(partial_sum);
            }
        });
        thread_handles.push(handle);
    }

    let table_mut_tickets = generate_tickets(partition_count_table.clone()).unwrap();

    for (ticket, sender) in table_mut_tickets
        .into_iter()
        .zip(sender_pool.iter().cycle())
    {
        sender.send(ticket).expect("Thread ded :(");
    }

    // we must drop the sender pool or else none of the threads will ever finish
    // as they will be blocked waiting for another message from the mpsc
    drop(sender_pool);

    for handle in thread_handles {
        handle.join().expect("thread died unexpectedly :(");
    }

    partition_count_table
        .get(n)
        .as_ref()
        .expect("didn't calculate up to n somehow")
        .clone()
}

// #[cfg(test)]
// mod tests {
//     const ANSWER_666: &str = "11956824258286445517629485";

//     use crate::BigInt;
//     use crate::{calc_partition_count, calc_partition_count_parallel};

//     #[test]
//     #[ignore]
//     fn serial_correct() {
//         let true_answer: BigInt = ANSWER_666.parse().unwrap();
//         let serial_answer = calc_partition_count(666);
//         assert_eq!(serial_answer, true_answer);
//     }

//     // #[test]
//     // fn parallel_correct() {
//     //     let true_answer: BigInt = ANSWER_666.parse().unwrap();
//     //     let parallel_answer = calc_partition_count_parallel(666, 12);
//     //     assert_eq!(parallel_answer, true_answer);
//     // }

//     #[test]
//     #[cfg(loom)]
//     fn loom_parallel_correct() {
//         loom::model(|| {
//             let true_answer: BigInt = ANSWER_666.parse().unwrap();
//             let parallel_answer = calc_partition_count_parallel(666, 12);
//             assert_eq!(parallel_answer, true_answer);
//         });
//     }
// }
