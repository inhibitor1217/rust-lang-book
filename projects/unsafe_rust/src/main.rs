use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

unsafe trait Foo {}

unsafe impl Foo for i32 {}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let addr = 0x012345usize;
    let r = addr as *mut i32;

    // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // println!("values is: {:?}", values);

    // println!("r1 is: {}", *r1);

    unsafe {
        println!("*{:?} is: {}", r1, *r1);
        println!("*{:?} is: {}", r2, *r2);

        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let (a, b) = split_at_mut(r, 2);

    assert_eq!(a, &mut [1, 2]);
    assert_eq!(b, &mut [3, 4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
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

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
