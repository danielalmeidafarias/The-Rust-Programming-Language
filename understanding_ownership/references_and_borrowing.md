# References and Borrowing
* Sometimes move behavior can be inconvenient. Programs often need to use a string more than once. 

## References are non-owning pointers
* The __&__ is used to create a reference to a pointer
* References does not have the data they point to
* After used, no data is deallocated

## Dereferencing a Pointer Acces Its Data
* Dereferencing operator __*__

## Rust avoids simultaneous aliasing and mutation

### Pointer Safety Principle: 
- Data should never be aliased and mutaded at the same time.

## References change permissions on paths
