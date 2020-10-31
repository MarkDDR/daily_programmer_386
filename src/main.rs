// https://old.reddit.com/r/dailyprogrammer/comments/jfcuz5/20201021_challenge_386_intermediate_partition/
// Partition Counts

mod multithreaded;

use rug::Integer;
use multithreaded::*;

fn main() {
    let target_n = 666_666;
    // let answer = calc_partition_count(target_n);
    // println!("serial:   {:?}", answer);

    let answer_parallel = calc_partition_count_parallel(target_n);
    println!("parallel: {:?}", answer_parallel);
}

#[derive(Copy, Clone, Debug)]
pub enum AddOrSub {
    Add,
    Sub,
}

pub fn generate_index_subtractions(up_to: usize) -> Vec<(usize, AddOrSub)> {
    let mut part_subs = vec![];
    let mut i = 0;
    loop {
        let mut n: i64 = (i + 2) / 2;
        if i % 2 == 1 {
            n = -n;
        }
        n = n * (3 * n - 1) / 2;
        if n > up_to as i64 {
            break;
        }
        let add_sub = if (i/2) % 2 == 0 {
            AddOrSub::Add
        } else {
            AddOrSub::Sub
        };
        part_subs.push((n as usize, add_sub));
        i += 1;
    }
    part_subs
}

fn calc_partition_count(n: usize) -> Integer {
    let index_subtractions = generate_index_subtractions(n);
    let mut index_subtraction_take_count = 2;

    let mut partition_count_table = vec![Integer::from(1), Integer::from(1)];

    for index in 2..=n {
        if index_subtractions.get(index_subtraction_take_count).map(|x| x.0) == Some(index) {
            index_subtraction_take_count += 1;
        }
        // dbg!(index, index_subtraction_take_count);
        let index_sub_slice = &index_subtractions[0..index_subtraction_take_count];

        let next_partition_count = index_sub_slice.iter()
            .rev()
            .map(|(x, add_or_sub)| {
                match add_or_sub {
                    AddOrSub::Add => partition_count_table[index - x].clone(),
                    AddOrSub::Sub => -1 * partition_count_table[index - x].clone(),
                }
            })
            .fold(Integer::from(0), |acc, x| acc + x);

        partition_count_table.push(next_partition_count)
    }

    partition_count_table[n].clone()
}