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
    * drop() is called automatically when the owner goes out of scope
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

A String is made up of three parts:
1. pointer *to the memory that holds the contents of the String*
1. length of the String - *how much memory in bytes the String is currently using*
1. capacity - *total amount of memory in bytes that the String has received from the OS*

NOTE: when dealing with fixed size data types (stored on the stack) when you make an assignment
of one fixed size data type to another Rust makes a copy.

let x = 5; // allocated on stack
let y = x; // copy of x, allocated on stack

let msg = String::from("hello");
let greeting = msg; // greeting does not contain the data for msg (only ptr, length, capacity
from the stack are copied)
* data on the heap that the ptr refers to is **NOT** copied
* helps prevent double free (msg goes out of scope and then greeting)
* **freeing memory** twice can lead to memory corruption and security vulnerabilities

In other languages a *shallow copy* is the process of copying the ptr, len and capacity
without copying the data. In **Rust** this is called a move since once we make that copy
rust invalidates the first variable as ownership has changed. (msg was moved into greeting)

Rust **does not** perform *deep copies*, automatic copying can be assumed to be 
to be inexpensive in terms of runtime performance.

If you want to perform a *deep copy* in Rust you will need to use clone()

```
let msg1 = String::from("hello");
let greeting = msg1.clone(); // potentially expensive

println!("msg1 = {}, greeting = {}", msg1, greeting);
```

NOTE: Generally any group of simple scalar values can have the *Copy* trait.
* i8, i16, i32, i64, i128, isize
* u8, u16, u32, u64, u128, usize
* bool
* f32, f64
* char
* tuples if they contain types that are also Copy

#### Ownership & Functions
When passing variables to a function the same Copy / Move mechanics are used.

```
fn main() {
    let s = String::from("hello"); // s into scope
    take_ownership(s); // s moved into function, no longer valid
    
    let x = 5; // x into scope
    makes_copy(x); // i32 is a Copy, can be used after function call

}

fn take_ownership(msg: String) { // msg into scope
    println!("{}", msg);
} // msg out of scope, drop() is called memory freed

fn makes_copy(num: i32) { // num into scope
    println!("{}", num);
} // num goes out of scope
```

#### References & Borrowing
Let's rewrite the *take_ownership* function to use a reference and borrow the variable
passed to it without taking ownership and rename it to display_message

```
fn display_message(msg: &String) {
    println!("{}", msg);
}
```
* The '&' used above are referred to as *references* and allow you to referr to
some value in Rust and not take ownership.
* the '*' is referred to as the *dereferencing operator*  
* *&s1* refers to the value of s1

References as function parameters is called *borrowing*.
* If you want to modify the value the reference points to, you must have
a mutable reference

```
fn display_message(msg: &mut String) {
    msg.push_str(" on Tuesday");
}
```
**NOTE:** in Rust you can not mutate any time you want to. This helps prevent data
races. A *data race* happens when these three behaviors occur:
* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There's no mechanism being used to synchronize access to the data.

Data Races cause *undefined behavior* and can be difficult to trouble-shoot. Rust prevents
this from occurring as the code will not compile.

Another rule Rust implements when dealing with references is you cannot have a mutable
reference at the same time you have an immutable reference. 

You can have multiple immutable references in Rust. A reference's scope starts at the 
point it was introduced and continues until the last point it was used.

Rust guarantees that references will never be *dangling pointer*. Occurs when 
a pointer reference a location in memory that has already been to given to something else, by 

```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello"); // when function ends s will be deallocated

    &s // would point to an ivalid String once fuction completes execution
}
```

* To fix the issue above, return the String directly
```
fn dangle() -> &String {
    let s = String::from("hello"); 

    s 
}
```

#### Slice Type (another reference)
* another data type in Rust that does not have ownership
* slices reference a contiguous sequence of elements in a collection rather than
the whole collection.

```
let s = String::from("hello world");

let hello = &s[..5];
let world = &s[6..];
let hello_world = &s[..];
```
**NOTE:** String slice range indices must land at valid UTF-8 character boundaries.

```
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

```
NOTE: string literals are slices.
* String literals are immutable and so is &str (immutable reference)

```
let s = "Hello World!"); // type &str slice point to a specific location in the binary
```

Lastly, slicing works on other data types as well. Let's look at slices and arrays.
```
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```
