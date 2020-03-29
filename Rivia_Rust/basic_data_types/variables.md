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
