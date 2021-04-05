#[cfg(not(loom))]
pub(crate) use std::{
    hint,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc, Arc,
    },
    thread,
};

#[cfg(loom)]
pub(crate) use loom::{
    hint,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc, Arc,
    },
    thread,
};
