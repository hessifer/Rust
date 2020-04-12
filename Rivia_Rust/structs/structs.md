### Struct
* structs are used to structure related data into meaningful groups.
* structs are like an object's data attributes.
* values within a struct can be of different types, similar to tuples.
* structs allow you to name your data.
* since structs can be named values you do not need to rely on the order as you do with tuples.
* A struct's name should describe the significance of the pieces of data being group together.
* *fields* are referred to the pieces of data contained within the struct.

```
// define a struct of type User
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

Once you have defined a struct you can create an *instance* of it and specifying values for each field.
* you do not have to specify the fields in the same order as they appear in the struct.

```
// create instance of User
let user1 = User {
    email: String::from("doesnot@exist.com"),
    username: String::from("doesnot"),
    active: true,
    sign_in_count: 1,
};
```

* You can use dot notation to get a specific value from the struct
* If the instance of the struct is mutable you can change the value by using dot notation as well.

```
// mutable instance
let mut user2 = User {
    email: String::from("blahblah@blahblah.com"),
    userrname: String::from("wazzupblah"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremailblahblah@blahblah.com");
```
**NOTE** - entire instance of struct must be mutable as it is not possible to make one field in the struct mutable.

When creating a function to build an instance of a struct if the parameters of the function contain the same name as
the fields in the struct, you can use *field init shorthand* syntax.

```
// field init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

With Rust, if you have an instance of a struct where you want to reuse most of that instance's values you can
use the *struct update syntax* feature.

**NOTE** we use String vs &str as we want to make sure that the data in our struct is valid for the life
of the struct. (*we will cover traits later to enable us to assign references as values*)
```
// struct using same values as user1 instance
let user3 = User {
    email: String::from("123@email.com"),
    username: String::from("supfewl"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};


// struct using update instead
let user4 = User {
    email: String::from("456@email.com"),
    username: String::from("nagalot"),
    ..user1
};
```

#### Tuple Struct
You can define structs that look similar to tuples, called *tuple structs*.
* Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a
different type than the other tuples, and naming each field as in a regular struct would be verbose.
* Tuple structs do not use named fields.

```
// two different types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

* You can destructure a tuple struct as you would a tuple (into their individual pieces of data.)
* You can use a '.' followed by an index to access an individual value


#### Unit Struct
* Unit structs do not have any fields. *unit-like structs* because they behave similarly to (), the unit type.
* Used to implement a *trait* on some type but don't have any data that you want to store in the type.


*NOTE* Rust has a feature called *automatic referencing and dereferencing*. If you call a method with
object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method.

* the following are the same
```
p1.distance(&p2);
(&p1).distance(&p2);
``` 

Rust has another useful feature called *associated functions*. These functions are associated to the struct; however, 
they do not take an instance of the struct as a parameter. They are called functions and not methods as they 
do not have an instance of the struct to work with.
* An example of an associated function is *String::from()*
* Associated functions are often used for constructors that return a new instance of the struct.

Rust allows structs to use multiple *impl* blocks 