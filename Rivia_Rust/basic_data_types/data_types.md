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
* Rust 'char' is 4 bytes in size and represents a Unicode Scalar Value

### Compound Types
* group multiple values into one type
* Rust has two primitive compound types:
    * tuples
    * arrays

#### Tuple
* group together a number of values of different types into one compound type
* tuples have a *fixed* length and cannot change once declared
* to declare a tuple use comma separated values inside of parenthesis ```let my_tuple: (u8, f64, str) = (1, 2.4, "yellow");```
* to destructure or unpack a tuple you would do something similar to: ```let (count, percent, color) = my_tuple;```
    * in addition to *destructuring* a tuple you can access its values using '.' followed by the index of the value you
    wish to retrieve. ```let my_color = my_tuple.2;```

#### Array
* a collection of multiple values
* the values stored in the array **must all be of the same type**
* arrays in Rust have a **fixed** length and cannot grow or shrink in size post declaration
* to declare an array use comma separated values inside of square brackets 
```let days_of_week: [str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];```
* an array is a single chunk of memory allocated on the stack (**we know the size at compile time**)
* to access the values of an array use their index like: ```let today = days_of_week[0];```
    * remember elements of an array are *zero-based* meaning that start with 0
* NOTE: Rust will prevent you from attempted to access an array element greater than or equal to the array
length.
