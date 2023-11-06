# References and Borrowing
* Sometimes move behavior can be inconvenient. Programs often need to use a heap variable (e.g a string) more than once. 

## References are non-owning pointers
* The __&__ is used to create a reference to a pointer
* References does not have the data they point to
* After used, no data is deallocated

## Dereferencing a Pointer Acces Its Data
* Dereferencing operator __*__
* Examples:

```rust
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; // *x reads the heap value, so a = 1
    *x += 1;         // *x on the left-side modifies the heap value, 
                     // so x points to the value 2

    let r1: &Box<i32> = &x; // r1 points to x on the stack
    let b: i32 = **r1;      // two deferences get us to the heap value

    let r2: &i32 = &*x; // r2 poits to the heap value directly
    let c: i32 = *r2;   // so only one dereference is needed to read it
``` 
- You probably won't see the dereference operator very often when you read Rust code. Rust implicitly inserts dereferences and references in certain cases with the dot operator


## Rust avoids simultaneous aliasing and mutation
- When you modify a Box, Rust will create a new allocation with larger capacity, copy all the elements over, and deallocate the original heap.
- If a variable references a changed Box, it will reference to a deallocated data.

### Pointer Safety Principle: 
- Data should never be aliased and mutaded at the same time.

## References change permissions on paths
- The core ideia of borrow checker is that variables have three kinds of permissions at compile-time: 
    1. Read(R): Data can be copied to another location
    2. Write(W): Data can be mutades in-place (mut variables)
    3. Own(O): Data can be moved or dropped

* Example: 
```rust
    let mut v: Vec<i32> = vec![1, 2, 3]; 
    //  v   -> +R +W +O

    let num: &i32 = &v[2]; 
    //  v   ->  R -W -O
    // num  -> +R  - +O
    // *num ->  R  -  -

    println!("Third element is {}", *num); 
    //  v   ->  R +W +O                              
    // num  -> -R  - -O
    // *num -> -R  -  - 

    v.push(4); 
    //  v   -> -R -W -O
```
* It's different to acces data through a reference versus to manipulate the reference itself
* When a path become unused it lose permissions because some permissons are mutually exclusive. For example __num = &v[2]__ then __v__ cannot be mutaded or dropped while, __num__ is in use

## The Borrow Checker finds permission violations
* The goal of these permissions is to ensure that data cannot be mutaded if it is aliased: __Pointer Safety Principle__

## Mutable References Provide Unique and Non-Owning Access to Data

```rust
    let mut v: Vec<i32> = vec![1, 2, 3];
    // v -> +R +W +O

    let num: &mut i32 = &mut v[2];
    //   v  -> -R -W -O
    //  num -> +R  - +O
    // *num -> +R +W  -

    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
    // v -> -R -W -O
```
* When num was an immutable reference, v still had the R permission. Now that num is a mutable reference, v has lost all permissions while num is in use.
* When num was an immutable reference, the path *num only had the R permission. Now that num is a mutable reference, *num has also gained the W permission.

## Permissions are Returned at the end of a Reference's Lifetime

## Data Must Outlive All of its References