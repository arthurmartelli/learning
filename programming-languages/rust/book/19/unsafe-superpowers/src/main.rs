use std::slice;

fn main() {
    raw_pointer();
    call_extern();
    modify_static();
    modify_static(); // Count will be 2
}

/// # Dereference a raw pointer
///
/// - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
/// - Aren’t guaranteed to point to valid memory
/// - Are allowed to be null
/// - Don’t implement any automatic cleanup
fn raw_pointer() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    drop(num);

    unsafe {
        // we can't dereference a raw pointer safely
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

/// # Call an unsafe function or method
fn call_unsafe(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // creating an unsafe function
    unsafe fn danger() {}

    // to be called, the function needs to be inside an unsafe block
    unsafe { danger() }

    // - safe abstraction to unsafe functions
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn call_extern() {
    extern "C" {
        // use `rustc --print=calling-conventions` to see all callable externs
        // we are able to call C code and libraries
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // to use from other languages we can do it like
    #[no_mangle] // will not change the functions name
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
}

/// Access or modify a mutable static variable
fn modify_static() {
    static mut HELLO_WORLD: &str = "Hello, world!";
    static mut COUNT: i32 = 0;

    unsafe {
        COUNT += 1;
        println!("name is: {}", HELLO_WORLD);
        println!("counter is: {}", COUNT);
    }
}

/// Implement an unsafe trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

/// Access fields of unions
fn dd() {}
