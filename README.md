# Rust Undefined Behavior Example

This repository contains a simple Rust program that demonstrates undefined behavior caused by multiple mutable references to the same variable without proper synchronization.

The `bug.rs` file contains the buggy code.  Running this code will produce unpredictable results depending on the compiler and runtime environment.

The `bugSolution.rs` file shows a corrected version using techniques to avoid this issue.  This could involve using interior mutability with types like `RefCell` or `Mutex`, or refactoring the code to avoid multiple mutable borrows.