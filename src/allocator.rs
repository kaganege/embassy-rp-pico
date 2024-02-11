extern crate alloc;

pub use alloc::*;
use embedded_alloc::Heap;

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();
