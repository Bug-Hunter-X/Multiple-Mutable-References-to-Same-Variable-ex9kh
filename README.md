# Multiple Mutable References in Rust

This repository demonstrates a common error in Rust related to mutable references.  The code in `bug.rs` shows how having multiple mutable references to the same variable can lead to unexpected results. The solution (`bugSolution.rs`) shows how to address the issue using techniques like cloning or using interior mutability with types such as `RefCell` or `Mutex`.

## Bug Description

Rust's borrow checker usually prevents issues with multiple mutable references. However, there are cases where multiple mutable references are possible but can lead to data races or unpredictable behavior. This example highlights one such instance. The order of modifications made through these references might not always be consistent.