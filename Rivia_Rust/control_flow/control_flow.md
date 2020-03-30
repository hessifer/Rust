### Control Flow
* many programs rely on conditions to determine which parts of code should execute
* The common construct that let you control the flow of execution of Rust code are if expressions and loops
* With *if* expressions you can branch your code so that if a particular condition is met the code will execute
* if conditions start with the keyword **if** followed by a condition
    * Rust expects each condition to evaluate to true of false
```
let age = 27;

if age > 21 {
    println!("Looks like you are allowed to drive.");
} else {
    println!("Hang in there, you will be behind the wheel in no time.");
}
```
* the blocks of code associated to a particular condition are referred to as **arms** in Rust
* to handle multiple conditions Rust uses the **else if** keywords
```
let age = 30;

if age < 13 {
    println!("You are a pre-teen.");
} else if age < 20 {
    println!("You are a teen.");
} else if age < 40 {
    println!("You are in your prime.");
} else {
    println!("You are over the hill.");
}
```
* **NOTE:** Using too many 'else if' arms can clutter your code and make it difficult to follow. 
If you are using more than one or two consider refactoring your code.
    * **refactoring** or restructuring your code for performance reason or to make it more legible without 
altering its behavior.
* Remember that 'if' is an expression so we can use it in variable assignment (to the right of the assignment 
operator '-').
```
let num = 6;
let is_even = if num % 2 == 0 {
    true
} else {
    false
};
```

### Repetition with Loops

#### loop
* Rust uses three types of loops: *loop, while, for*
* loop keyword tells Rust to execute a loop forever or until a break statement is found
```
loop {
    println!("I will execute forever.....yeeeeeeeee!");
}

let mut counter = 0;

loop {
    println!("Keep going until I reach a counter of 3.");
    counter += 1;

    if counter == 3 {
        break;
    }
}
```
#### while
* allows you to execute code while a particular condition is true or until we reach a break statement
```
let mut counter = 3;

while counter > 0 {
    println!("{}", counter);
    counter -= 1;
}
println!("Blast off!");
```
#### for
* used to loop through collections
* Rust for loops are safe and concise which makes them the most commonly used repetition control
```
let colors = ["yellow", "blue", "orange"];

for color in colors.iter() {
    println!("{}", color);
}
```
* for loops can also be used to execute code a specific number of times
    * to do this we can use a range
```
for i in (1..4).rev() {
    println!("{}", i);
}
println!("Blast off!");
```

