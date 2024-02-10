extern crate alloc;

use alloc::alloc::*;

#[derive(Default)]
pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    malloc(layout.size() as u32) as *mut u8
  }
  unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
    free(ptr as *mut c_void);
  }
}

/// If there is an out of memory error, just panic.
#[alloc_error_handler]
fn my_allocator_error(_layout: Layout) -> ! {
  panic!("out of memory");
}

/// The static global allocator.
#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;
