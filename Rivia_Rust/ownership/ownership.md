### Ownership
Rust does not implement a garbage collector, a feature that constantly 
looks for no longer used memory as the program executes. In some programming
languages the programmer must manage the memory through allocation and deallocation
techniques. Having said that Rust uses **ownership**, Rust's most unique feature
that allows Rust to make memory safe guarantees.

Understanding Rust ownership is critical to becoming a solid Rustacean! Rust
uses ownership along with a set of rules that the compiler checks at compile time.
Ownership does not slow down your program during execution.

#### Stack & Heap
The stack stores values in the order it gets them and removes the values in the
opposite order. It uses a last in, first out (LIFO) technique. Think about a public
cafe where you store food trays. You push the tray onto the stack of existing trays
and remove the last one off the stack referred to as **popping off the stack**. Any
data stored on the stack must have a *known fixed size* at compile time. Data that is
not of a fixed size must be stored on the **heap**. The heap is not as structured/
organized as the stack. When you store data on the heap the code request a specific
amount of space on the heap. The OS will find an empty spot big enough to service 
the request and marks that space as being used and then returns a **pointer** or
memory address of that location. Here we are *allocating on the heap*. The pointer
is stored on the stack since it is a known fixed size. The access the data you must
follow the pointer to the location in heap.

Pushing onto the stack is **faster** than allocating on the heap. When allocating on the
heap the OS has to search for a place to store new data and perform bookkeeping to prepare
for the next allocation request. **Accessing** data in the heap is **slower** than 
accessing data on the stack because you have to follow a pointer to get there. 
Newer processors are faster if they jump around less in memory. Working with data
that is close to other data, like in the stack for example is a quicker more efficient
process.

When working with functions, local variables, function arguments and pointers get stored
on the stack. After the function has completed execution the values are popped off the
stack. Ownership is responsible for keeping track of what parts of code are using what
data on the heap, minimizing the amount of duplicate data on the heap and cleaning up
unused data ont he heap so you don't run out of space are all problems that ownership
addresses. Managing heap data is why ownership exist.

#### Ownership Rules
* Each value in Rust has a variable that's called its *owner*
* There can only be one owner at a time
* When the owner goes out of scope, the value will be dropped

*scope* space within a program for which a variable is valid.
```
// string literal - text compiled into the binary (stack) fast and effecient
// string literals are immutable
let msg: str = "hello";
```

The **String** data type in Rust is allocated on the heap.
```
// String data type which can be mutable as it is stored on the heap.
let mut s = String::from("hello");

s.push_str(", world"); // append string literal value in quotes to s
println!("{}", s); // prints hello, world
```
