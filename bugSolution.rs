fn main() {    let mut x = 5;    let y = &mut x;    *y = 6;    println!("x = {}", x); // Output: x = 6    // Solution 1: Cloning
    let mut z = *y; //clone the value
    z = 7;
    println!("z = {}", z); // Output: z = 7
    println!("x = {}", x); // Output: x = 6

    // Solution 2: Using RefCell (for interior mutability)
    use std::cell::RefCell;

    let x = RefCell::new(5);
    let y = x.borrow_mut();
    *y = 6;
    println!("x = {}", x.borrow());
    let z = x.borrow_mut();
    *z = 7;
    println!("x = {}", x.borrow());
} 