use std::sync::atomic::{AtomicUsize, Ordering, AtomicBool};
use std::sync::Arc;
use std::ptr::NonNull;
use rug::Integer;
use crate::{AddOrSub, generate_index_subtractions};
use threadpool::ThreadPool;

pub struct ParallelTableMutIndexTicket {
    index: usize,
    table: Arc<ParallelTable>,
}

impl ParallelTableMutIndexTicket {
    fn get_index(&self) -> usize {
        self.index
    }

    fn set(self, value: Integer) {
        assert!(self.table.capacity > self.index);
        // SAFETY: this function is memory safe because each index only gets a single ticket
        unsafe {
            (*self.table.shared_mem_ptr.as_ptr())[self.index] = Some(value);
        }
        // We could just use store instead of swap, but I want to include a sanity check on old index
        // self.table.valid_to_store(self.index, Ordering::Release);
        let old_valid = self.table.valid_to.swap(self.index, Ordering::AcqRel);
        // due to the algorithm we use, we can't possibly have a value to store unless we know all values up to the previous one
        assert_eq!(self.index, old_valid + 1);
    }
}

#[derive(Debug)]
pub struct ParallelTable {
    shared_mem_ptr: NonNull<[Option<Integer>]>,
    valid_to: AtomicUsize,
    capacity: usize,
    tickets_generated: AtomicBool,
}

impl ParallelTable {
    pub fn new(size: usize) -> Self {
        let mut shared_mem = vec![None; size+1];

        shared_mem[0] = Some(Integer::from(1));
        shared_mem[1] = Some(Integer::from(1));


        Self {
            shared_mem_ptr: NonNull::new(shared_mem.leak()).unwrap(),
            valid_to: AtomicUsize::new(1),
            capacity: size+1,
            tickets_generated: AtomicBool::new(false),
        }
    }

    pub fn get(&self, index: usize) -> &Option<Integer> {
        let valid = self.valid_to.load(Ordering::Acquire);
        if index <= valid {
            unsafe {
                let slice = self.into_slice(valid);
                &slice[index]
            }
        } else {
            &None
        }
    }

    pub fn get_all_valid(&self) -> &[Option<Integer>] {
        let valid = self.valid_to.load(Ordering::Acquire);
        unsafe {
            self.into_slice(valid)
        }
    }

    /// # SAFETY
    /// This function does not check is size is valid
    unsafe fn into_slice(&self, size: usize) -> &[Option<Integer>] {
        std::slice::from_raw_parts(self.shared_mem_ptr.as_ptr() as *const _, size + 1)
    }
}

pub fn generate_tickets(parallel_table: Arc<ParallelTable>) -> Option<Vec<ParallelTableMutIndexTicket>> {
    let old_value = parallel_table.tickets_generated.compare_and_swap(false, true, Ordering::AcqRel);
    if !old_value {
        let tickets: Vec<_> = (2..parallel_table.capacity)
            .map(|index| ParallelTableMutIndexTicket { index, table: parallel_table.clone() })
            .collect();
        Some(tickets)
    } else {
        None
    }
}

unsafe impl Sync for ParallelTable {}
unsafe impl Send for ParallelTable {}

pub fn calc_partition_count_parallel(n: usize) -> Integer {
    let index_subtractions = Arc::new(generate_index_subtractions(n));
    let partition_count_table = Arc::new(ParallelTable::new(n));

    let num_threads = num_cpus::get();
    let pool = ThreadPool::new(num_threads);

    let table_mut_tickets = generate_tickets(partition_count_table.clone()).unwrap();

    for ticket in table_mut_tickets {
        let partition_count_table = partition_count_table.clone();
        let index_subtractions = index_subtractions.clone();

        pool.execute(move || {
            let mut index_subtraction_take_count = 2;
            for (i, (sub_amt, _)) in index_subtractions.iter().enumerate().skip(2) {
                if ticket.get_index() >= *sub_amt {
                    index_subtraction_take_count = i+1;
                } else {
                    break;
                }
            }

            let index_sub_slice = &index_subtractions[0..index_subtraction_take_count];
            let part_valid = partition_count_table.get_all_valid();
            let mut partial_sum = Integer::new();

            for (sub_amount, add_or_sub) in index_sub_slice.iter().rev() {
                let get_index = ticket.get_index() - sub_amount;
                let int: &Integer = match part_valid.get(get_index) {
                    Some(int) => int.as_ref().unwrap(),
                    None => {
                        loop {
                            match partition_count_table.get(get_index) {
                                Some(int) => break int,
                                None => {},
                            }
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

    // just for the debugger
    // let x = Arc::try_unwrap(partition_counts).expect("poo");
    // x.get(n).as_ref().expect("didn't calc n").clone()

    // let l = partition_counts.get_all_valid();
    // for (i, e) in l.iter().enumerate() {
    //     println!("{}: {}", i, e.as_ref().unwrap());
    // }
    partition_count_table.get(n).as_ref().expect("didn't calculate up to n somehow").clone()
}