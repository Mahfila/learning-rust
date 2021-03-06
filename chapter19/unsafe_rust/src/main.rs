extern "C" {
    fn abs(input: i32) -> i32;
}

use std::slice;

static mut COUNTER: u32 = 0;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;


    unsafe {
        *r2 += 3;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let mut numbers = vec![1,2,3,4,5,6,7];
    let (a, b) = split_at_mut(&mut numbers, 4);
    println!("a: {:?}, b: {:?}", a, b);

    unsafe {
        println!("abs of -3 according to C: {}", abs(-3));
    }

    update_counter(1);
    unsafe {
        println!("The counter now is: {}", COUNTER);
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let size = slice.len();
    let pointer = slice.as_mut_ptr();

    // assert!(mid <= size);

    unsafe{(
        slice::from_raw_parts_mut(pointer, mid),
        slice::from_raw_parts_mut(pointer.offset(mid as isize), size - mid),
    )}
}

fn update_counter(increase_by: u32) {
    unsafe {
        COUNTER += increase_by;
    }
}