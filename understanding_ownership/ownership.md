# What is Ownership?

## Safety is the Absence of Undefined Behavior
* A foundational goal of Rust is to ensure that your programs never have undefined behavior. That is the meaning of "safety." Undefined behavior is especially dangerous for low-level programs with direct access to memory.

* A secondary goal of Rust is to prevent undefined behavior at compile-time instead of run-time. This goal has two motivations:
    - Catching bugs at compile-time means avoiding those bugs in production, improving the reliability of your software.
    - Catching bugs at compile-time means fewer runtime checks for those bugs, improving the performance of your software.

## Ownership as a Discipline for Memory Safety
* Variables lives in the __Stack__
* Boxes live in the __Heap__

```rust
let a = Box::new([0; 1_000_000]);
let b = a;

```

* The stack holds data associated with a specific function, while the heap holds data that can outlive a function

* Rust does not permit manual memory management

## A Box's Owner Manages Deallocation
* Rust automatically frees a box's heap memory

### Box deallocation principle:
- If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.

## Collections Use Boxes
* Boxes are used by Rust data structures like __Vec__, __String__ and __HashMap__
* Variables cannot be used after being moved

### Moved heap data principle:
* if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.

## Cloning avoids moves
* One way to avoid moving data is to clone it using the __.clone()__ method

## Sumary
1. All heap data must be owned by exactly one variable.
2. Rust deallocates heap data once its owner goes out of scope.
3. Ownership can be transferred by moves, which happen on assignments and function calls.
4. Heap data can only be accessed through its current owner, not a previous owner.