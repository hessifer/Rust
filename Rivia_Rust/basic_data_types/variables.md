## Variables
**NOTE:** variables in rust are *immutable* by default. This means that by default you cannot change the value of a
variable once it has been assigned.

**NOTE:** as with most languages Rust uses *keywords* tht have special meaning to the language and should not be used
as variable or function names.

1. To make a variable mutable you must add the keyword *mut* in front of your variable name. As an
example: ``let mut age = 27``
    * this will be become more important as you use larger data structures that you need to mutate in place.
1. Rust also makes constants available to you with the keyword *const*.
    * *const* differ from immutable variables as *const* are always immutable and must be accompanied by their type
    annotation. 
    * *const* may be declared in any scope including the global scope.
    * an example of a *const* declaration is: ```const HIGH_SCORE: u32 = 1_235_000```
        * **note** by convention you should use '_' to separate words and to make numbers more legible.
    * constants are valid for the lifetime of the program
1. In Rust developers can declare variables in different scopes to use the same name. *Scope* is referred to as the
the area for which a variable and its value is valid (able to be used). As the execution of a program continues the
section of code that executes progresses and an outer scope variable with the same name of an inner scope variable will
be **shadowed** by that inner scope variable. An example of this:
```
fn main() {
    let my_age: u8 = 27;
    println!("OUTTER SCOPE");
    println!("Hello my age is {}.", my_age);
    
    loop {
        let my_age: u8 = 30;
        println!("INNER SCOPE");
        println!("OUTTER SCOPE is 'shadowed' by INNER SCOPE");
        println!("Hello my age is {}.", my_age);
        break;
    }

    println!("OUTTER SCOPE");
    println!("Hello my age is {}.", my_age);
```

## Data Types
**NOTE:** Rust is a *statically typed language* meaning all values in Rust **must** have a type at compile time.

1. There two subsets of Data Types in Rust called **Scalar** and **Compound**.
### Scalar Types
* have a single value
* Rust has four Scalar types:
    * integers
    * floating-points
    * booleans
    * characters
#### Integers
* an integer is a number without a fractional part
* Integers can be either *signed* or *unsigned*
    * *unsigned* integers are positive values
    * *signed* integers are either positive or negative
    * i8 | u8
    * i16 | u16
    * i32 | u32
    * i64 | u64
    * i128 | u128
    * isize | usize
* signed variants can store numbers from ```-(2^n - 1) to 2^n - 1 - 1```
* unsigned variants can store numbers from ```0 to 2^n - 1```
* *n* represents the number of bits the variant uses.
* usize and isize is based on the architecture of the machine you are on. 
* integers default to *i32*
* NOTE: look into integer overflow and twos complement rule. Essentially when Rust sees a value greater than what
the integer type can hold it will either panic at compile time in Debug mode or **wrap** the value in Release mode.
Wrapping means that if you have a value of 256 stored in your variable of type u8, the value would wrap around to 
0 given the max value of an u8 type is 255.
    * ```0 to 2^8 - 1 = 256 - 1 = 255```

#### Floating-Points
* floating-points are numbers with decimal points.
* Rust has two primitive types for floating-points: *f32* and *f64*
    * f32 provides single precision
    * f64 provides double precision
* Rust supports all the basic Mathematical operations you would expect
    * *, +, /, %, -, 

#### Boolean
* one of two possible values true or false
* boolean are 1 byte in size


#### Character
* Rust most primitive type is the **char** type
* specified using single quotes ```let letter: char = 'c';```
* 
    