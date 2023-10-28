# What is Ownership?

## Safety is the Absence of Undefined Behavior
* A foundational goal of Rust is to ensure that your programs never have undefined behavior. That is the meaning of "safety." Undefined behavior is especially dangerous for low-level programs with direct access to memory.

* A secondary goal of Rust is to prevent undefined behavior at compile-time instead of run-time. This goal has two motivations:
    - Catching bugs at compile-time means avoiding those bugs in production, improving the reliability of your software.
    - Catching bugs at compile-time means fewer runtime checks for those bugs, improving the performance of your software.

## Ownership as a Discipline for Memory Safety
* Variables lives in the Frames
* Boxes live in the Heap

* The stack holds data associated with a specific function, while the heap holds data that can outlive a function

## A Box's Owner Manages Deallocation

### Box deallocation principle:
- If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.

## Collections Use Boxes
* Boxes are used by Rust data structures like __Vec__, __String__ and __HashMap__
* Variables cannot be used after being moved

### Moved heap data principle:
* if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.