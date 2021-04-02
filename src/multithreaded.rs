use crate::bigint::BigInt;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
// use crate::util::*;

pub struct ParallelTableMutIndexTicket {
    index: usize,
    table: Arc<ParallelTable>,
}

impl ParallelTableMutIndexTicket {
    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn set(self, value: BigInt) {
        assert!(self.table.capacity > self.index);
        // SAFETY: this function is memory safe because each index only gets a single ticket and other
        // threads are not allowed to observe this uninitalized memory
        unsafe {
            (*self.table.shared_mem_ptr.as_ptr())[self.index] = Some(value);
        }

        // SAFETY: We require self.table.valid_len == self.index before we can update
        // to prevent data races from being visible to the end user from mis-use.
        // For this project, we will panic if such a scenario happens. This will leave the other
        // threads in the threadpool in a deadlock since we don't handle thread panics yet
        self.table
            .valid_len
            .compare_exchange(
                self.index,
                self.index + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .expect("Something has gone terribly wrong");
    }
}

#[derive(Debug)]
pub struct ParallelTable {
    shared_mem_ptr: NonNull<[Option<BigInt>]>,
    valid_len: AtomicUsize,
    capacity: usize,
    tickets_generated: AtomicBool,
}

impl ParallelTable {
    pub fn new(target_n: usize) -> Self {
        let capacity = target_n + 1;
        let mut shared_mem = vec![None; capacity];

        shared_mem[0] = Some(BigInt::from(1));
        shared_mem[1] = Some(BigInt::from(1));

        Self {
            shared_mem_ptr: NonNull::new(shared_mem.leak()).unwrap(),
            valid_len: AtomicUsize::new(2),
            capacity,
            tickets_generated: AtomicBool::new(false),
        }
    }

    pub fn get(&self, index: usize) -> &Option<BigInt> {
        let valid = self.valid_len.load(Ordering::Acquire);
        if index < valid {
            unsafe {
                let slice = self.into_slice(valid);
                &slice[index]
            }
        } else {
            &None
        }
    }

    pub fn get_all_valid(&self) -> &[Option<BigInt>] {
        let valid = self.valid_len.load(Ordering::Acquire);
        unsafe { self.into_slice(valid) }
    }

    /// # SAFETY
    /// This function does not check if size is valid
    unsafe fn into_slice(&self, size: usize) -> &[Option<BigInt>] {
        std::slice::from_raw_parts(self.shared_mem_ptr.as_ptr() as *const _, size)
    }
}

unsafe impl Sync for ParallelTable {}
unsafe impl Send for ParallelTable {}

impl Drop for ParallelTable {
    fn drop(&mut self) {
        let boxed_slice: Box<[Option<BigInt>]> =
            unsafe { Box::from_raw(self.shared_mem_ptr.as_ptr()) };
        drop(boxed_slice);
    }
}

pub fn generate_tickets(
    parallel_table: Arc<ParallelTable>,
) -> Option<Vec<ParallelTableMutIndexTicket>> {
    let res = parallel_table.tickets_generated.compare_exchange(
        false,
        true,
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    match res {
        Ok(false) => {
            let tickets: Vec<_> = (2..parallel_table.capacity)
                .map(|index| ParallelTableMutIndexTicket {
                    index,
                    table: parallel_table.clone(),
                })
                .collect();
            Some(tickets)
        }
        Err(_) => None,
        unexpected => panic!("Unexpected return {:?}", unexpected),
    }
}
