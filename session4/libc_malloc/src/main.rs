fn allocate_memory_with_libc() {
    unsafe {
        // allocate memory with libc (one 32-bit integer)
        let my_num: *mut i32 = libc::malloc(std::mem::size_of::<i32>() as libc::size_t) as *mut i32;

        if my_num.is_null() {
            panic!("failed to allocate memory")
        }

        *my_num = 42;
        assert_eq!(42, *my_num);

        libc::free(my_num as *mut libc::c_void); // release memory
    }
}

fn allocate_memory_with_rust() {
    use std::alloc::{alloc, dealloc, Layout};

    unsafe {
        let layout = Layout::new::<u16>();

        let ptr = alloc(layout);

        // set the allocate variable - dereference the pointer and set to 42
        *ptr = 42;
        assert_eq!(42, *ptr);

        // Free to memory  - this is not automatic
        dealloc(ptr, layout);
    }
}

fn main() {
    // println!("Start program allocate with libc");
    // allocate_memory_with_libc();
    // println!("Ended program allocate with libc");

    println!("Start program allocate with rust native");
    allocate_memory_with_rust();
    println!("Ended program allocate with rust native");
}
