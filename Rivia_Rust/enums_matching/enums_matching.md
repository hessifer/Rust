### Enums
* Allow you to define a custom type by enumerating possible *variants*
* In Rust, enums are most similar to *algebraic data types* in functional languages like F#, OCaml, Haskell.

Let's take a look at creating a enum to represent the all the possible *variants* of an IP Address. An IP Address
can either by v4 or v6 but not both at the same time.
```
// custom data type
enum IpAddrVersion {
    v4,
    v6,
}

let four = IpAddrVersion::v4;
let six = IpAddrVersion::v6;

// define a function that takes an IpAddVersion type as a parameter.
fn route(ip_version: IpAddrVersion) {...}

// to call the function
route(four);
route(six);
```

With *enums* you have a way of storing the possible versions of an IP address; however, you cannot store
the actual data of the IP address. This is where the combination of structs and enums come in. You can use the 
enum to contain the possible variants of an IP address version and a struct to store the actual IP address data.

```
enum IpAddrVersion {
    v4,
    v6,
}

struct IpAddr {
    version: IpAddrVersion,
    address: String,
}

let home = IpAddr {
    kind: IpAddrVersion::v4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrVersion::v6,
    address: String::from("::1"),
};
```

We can represent the above code in a more concise way by attaching data to each possible variant directly.

```
enum IpAddr {
    v4(u8, u8, u8, u8),
    v6(String),
}

let home = IpAddrVersion::v4(String::from("127.0.0.1"));
let loopback = IpAddrVersion::v6(String::from("::1"));
```

Rust standard library already has a method for dealing with this.

```
struct Ipv4Addr {
    ...
}

struct Ipv6Addr {
    ...
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Like with structs, you can implement methods on enums.

```
enum Message {
    Quit,
    Move { x: i32, y: i32 };
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // body of method
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

#### Option enum

* The *option* enum is defined in Rust's standard library.
* With the *option* enum, values can either be something or nothing
* Rust does not have the *null* value as other programming languages do.
    - suggested reading *Null References: The Billion Dollar Mistake* by Tony Hoare (2009)
    - the problem with null values is when you try to use them as not-null values.
* The concept of a value being null isn't the problem, it is the implementation
* Rust uses the *option* enum to encode the concept of a value being present or absent.
* *option* enum is included in the prelude allows direct usage without bringing it into scope
* the Some and None can be used directly without the **Option::** prefix.
* Some(T) and None are variants of type **Option<T>**.
* **<T>** is a generic type parameter.
    - for know just know that it represents the fact that the **Some(T)** variant can hold one piece of data of any 
      type.

```
enum Option<T> {
    Some(T),
    None,
}

let some_number = Some(5);
let some_string = Some("a string");

// need to tell Rust what type of Option<T> will be used as it can't be inferred
let absent_number: Option<i32> = None;
```
* everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null.
 This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.
* additional reading: https://doc.rust-lang.org/std/option/enum.Option.html
    - learn the methods on *Option<T>*

#### Match
* a control flow construct that allows you to run some code when you have a Some(T) value and is allowed
to use the inner T. If you want some other code to run if you have *None* value and that code does not
have a *T* value available.

* Rust's powerful control flow operator called *match* allows you to compare a value against a series of 
patterns and the execute code based on which pattern matches.
    - NOTE: it's a first match wins algorithm

```
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

The Patterns and Code inside the {} are referred to as *arms* (pattern and code to execute). Arms are
separated by ','

A match can have as many arms as needed and must be exhaustive, meaning all possible patterns have been 
accounted for.

* The code associated with a arm is an expression, and the resulting value of the expression in the matching 
arm just returns a value in this example.

* If you need to run multiple lines of code in an arm, you need to use curly braces.

```
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

```
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter represents: {:?}!", state);
            25
        },
    }
}
```

Previously we looked at getting the inner *T* value out of *Some* when using *Option<T>*. No let's look
at how to handle Option<T> using *match*

```
// write a function that takes an *Option<i32>* and if there is a value inside add 1 to that value.
// If there is no value inside, the function should return the *None* value and not attempt
// to perform any operations
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

```

**Reminder** Matches in Rust are exhaustive, you must exhaust every possibility in order for the code to
be valid.

* The *_* patter will match any value. Putting this after your arms it will match all possible cases 
that aren't specified before it. The *()* below is just the unit value so nothing will happen. Essentially
we are saying below that we want to do nothing for all the possible values that we don't list before
the *_* placeholder.

``` 
let some_u8_value = 0u8;

match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

The *match* expression can be a bit verbose in a situation in which we care about only *one* of the cases.

- For this situation, Rust provides *if let*

#### if let
* allows you to combine if and let into a less verbose way to handle values that match one pattern while
ignoring the rest.
```
let some_u8_value = Some(0u8);

// in the match below we only care about one thing and discard the rest, use if let
match some_u8_value {
    Some(3) => println!("three");
    _ => (),
}
```

The example above rewritten to use an *if let*
```
if let Some(3) = some_u8_value {
    println!("three");
}
```

* The above example is cleaner and less verbose. We don't get the same exhaustive checking with if let
as we do with match, but this use case did not call for it.

* An *else* can be used in conjunction with *if let*, if you wanted a catch all like we had in our
match example you could:

```
if let Some(3) = some_u8_value {
    println!("three");
} else {
    println!("not three");
}
```
