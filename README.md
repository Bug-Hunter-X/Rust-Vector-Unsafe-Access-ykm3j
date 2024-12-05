# Rust `get_unchecked` Example and Safe Solution

This repository demonstrates an example of using the unsafe `get_unchecked` method in Rust to access elements of a vector.  It also provides a safe alternative using the `get` method.  This example highlights the importance of safe coding practices in Rust when dealing with potential out-of-bounds errors.

## Bug
The `bug.rs` file contains code that uses `get_unchecked` to access a vector element.  This is unsafe and can cause crashes or unpredictable behavior if the index is out of bounds.

## Solution
The `bugSolution.rs` file provides a safer alternative, using the `get` method to access the vector element.  The `get` method returns an `Option`, which allows handling cases where the index is out of bounds gracefully.
