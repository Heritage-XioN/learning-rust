# Learing Rust

## Data types

### Numbers - Integers
**signed:** can be positive or negative   
**unsigned:** can only be positive
| length | signed | unsigned |
|--------|--------|----------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u8       |
| 32-bit | i32    | u8       |
| 64-bit | i64    | u8       |
| 128-bit | i128  | u8       |
| arch | isize    | usize    |

### Numbers - floats
numbers with decimal points
| length | type    |
|--------|---------|
| 32-bit | f32     |
| 64-bit | f64     |

### char
single unicode scalar value
| length | type    |   use   |
|--------|---------|---------|
| 4-bytes | char   |   ' '   |

### string - String
dynamic strings scalar UTF-8 encoded that are heap allocated, mutable and are owned, it is stored on the heap with a pointer in the stack.
| length  | type    |                use               |
|---------|---------|----------------------------------|
| dynamic | String  |  " ".to_string() or String::from(" ");  |

### string - str
strings scalar UTF-8 encoded that are a borrowed immutable reference
| length | type    |   use  |
|--------|---------|--------|
| 4-bytes |  &str  |   " "  |

### unit type
this is an empty tuple of size/length 0 used to return nothing in expressions or functions
| length | type    |
|--------|---------|
|    0   |   ()    |

### heap allocated data types
|    length     |         type         |      usage      |
|---------------|----------------------|-----------------|
|    datatype   |   Box< datatype >    | Box::new(value) | 
```rust
let num: Box<u32> = Box::new(25);
```

### bool
truthy or falsy values
| length |   type  |     value     |
|--------|---------|---------------|
|    1   |   bool  | true or false |

### statement & expression

**statement:** instructions that perform some action but do not produce value. eg: functions or code that ends with ";"     
**experession:** instructions that evaluate to a resultant value 

*Eg:*
```rust
fn main(){
    let x: u32 = 5_u32;

    // the entire operation below is statement
    let y: u32 = {
        let x_squared = x * x; // statement
        let x_cubed = x_squared * x; // statement

        // this is an expression
        // and it will be assigned to y 
        x_cubed + x_squared + x
    }; // the { } it self is an expression because it 
     // produces a value at the end

    let z: () = {
        // the semicolon suppresses this expression
        //  and "()" is assigned to "z"
        2 * z;
    }
}
```
### functions & diverging function

**function:** this is a block of reusable code that performs a specific task, it can arguments, processes those inputs and returns a result  

**diverging function:** this is a function that never returns to the caller. eg: panic, looping forever, quitting the program

*Eg:*
```rust
// main function (entry point)
fn main(){
    let (x: i32, y: i32) = (1, 3);
    let s: i32 = sum(x, y); // function

    assert_eq!(s, 4);

    // anything below this will not be reached
    // comment it out to see the "success"
    never_return(); // diverging function call

    println!("Success")
}

// function definition, return an i32 value
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// diverging function returns nothing
fn never_return() -> ! {
    // this throws an error
    panic!()
}
```
#
## Ownership
rust ownership system is a set of rules that govern
memory management, these rules are enforced at compile
time and if any of them are violated the program won't 
run 

**owner:** the owner of a value is the variable or data
 structure that holds it, this owner is responsible for
 allocating and freeing the memory used to store that
 data

**Rules of ownership:**
1. Each value has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope the value will be dropped

### Scope
this is the range within a program for which an item is
valid  
**global scope:** accessible throughtout the entire
program  
**local scope:** only accessible within a particular functtion or block of code

*Eg:*
```rust
// the curly brackets is a scope
{
    let x = "Hello";// this is only valid within the "{}"

    // do something with x
}

// trying to access x outside the "{}" will result in an error
// so basically anything within the main function is
// globally scoped and can be used by any statement or
// expression called, used or declared within its 
// excution context (scope)
```

### Memory
this is component in a computer used to store data and
instructions for the processor to execute, that compoment is called the RAM.  

**Random Access Memory(RAM):** the RAM is volatile
 meaning when the power is turned off all contents are
lost. there a two main regions in a RAM used by a program at runtime
they are the **stack** and **heap**
memoru 

- **stack memory:** this follows the `last in`,
`first out` principle, all data stored on the stack must
have a known fixed size like integers, floats, char,
bool, etc. pushing to the stack is faster than
allocating to the heap because the location for the new
data is always at the top of the stack.  


- **heap memory:** this stores data of unknown or no fixed
size, allocating data to the heap returns a pointer(an 
address to the location where the data has been allocated). 
allocating and accessing data from the heap is slower than 
pushing or retrieving data from the stack

types of unknown size will get allocated to the heap and a 
pointer (this has a fixed size: `usize`) to the value is 
pushed to the stack.

**copy vs move:** scalar values(values with a fixed size) 
which are stored in the stack are copied instead of moved 
because this is computational cheap.

in the example below the integer value of the variable "x" is 
copied into "y" and both variables are usable. 
```rust
let x = 5;
let y = x;
```
this is because the variable "x" is assigned an integer value 
of type i32(it has a fixed size at compile time)

but in the example below since strings are dynamic scalar 
values that can not be stored in the stack, instead the s1 
variable stores a pointer to the inistial value and the 
length and the capacity of the string in the stack because 
all these are of size usize which resolves to a fixed size at 
compile time, 
```rust
let s1 = String::from("Hello");
let s2 = s1;
```
so when s2 = s1, s2 now points to the same memory address as 
s1, this violates rust second rule of ownership which states
there can only be one owner at a time. so what rust does is 
it will drop the first variable s1 therefore it cannot be 
accessed after assigning it to s2 this is done to avoid 
dangling pointers.

in situation where you still want to have access to the
variable "s1" you can explicitly tell the compile to perform 
a deep copy using the ".clone()" method, this clones the 
exact value of "s1" and assigns it to "s2"
```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```
**Note:** this could be computationally expensive depending 
on the size of data stored on the heap

**partial move:** within the destructuring of a single 
variable, both by-move and by-reference pattern bindings can 
be used at the same time. doing this will result in a partial 
move of the variable, which means that parts of the variable 
will be moved while other parts stay. in such a case the 
parent variable cannot be used afterwards as a whole, however 
the parts only referenced(not moved) can still be used.  

imagine this is a scenario where one or more of the values in
a data structure like a tuple is destructured and assigned to 
another variable, in this scenario the tuple losses ownership 
of that particular data that was assigned but it still 
retains ownership of the rest(unassigned or just referenced)

Eg:
```rust
fn main(){
    #[derive(Debug)]
    // custom data type
    struct Person {
        name: String,
        age Box<u8>,
    }

    let person: Person = Person {
        name: String::from("David"),
        age: Box::new(20),
    };

    // 'name' is moved out of person but 'age' is referenced
    let Person {name, ref age} = person; // Note the ref keyword
    // from this line the "person" variable losses ownership of its "name" member, but it still has access to "age"
}
```

**functions as it relates to ownership:**
```rust
fn main (){
    let s1 = String::from("hello"); // s1 comes into scope
    takes_ownership(s1); // takes_ownership() gains ownership of s1
    // therefore s1 is not valid from this line

    let num = 20; // num comes into scope
    make_copy(num); // num will move into the function
    // but because num is i32 the above operation is a copy
    // so it can still be used at this line 

    let s2 = gives_ownership(); // gives_ownership moves it 
    // return value to s2 making s2 the new owner

    let s3 = String::from("give"); // s3 comes into scope
    // s3 is moved into the takes_and_gives_ownership()
    let s4 = takes_and_gives_ownership(s3) 
    // which also moves it return value to s4 making s4 the owner
    // of "give" at compile time

}// s2, s4 goes out of scope and is dropped. s1, num, s3 are moved

fn takes_ownership(x: String){ // s1 as x comes into scope here
    println!("{}", x);
}// here x goes out of scope

fn make_copy(y: i32){ // num as y comes into scope here
    println!("{}", y);
}// here y goes out of scope

fn gives_ownership() -> String {
    let value = String::from("yours"); // value comes into scope
    value // value is returned and moves out to the calling function
}

// takes ownership and returns/gives ownership to the caller 
fn takes_and_gives_ownership(z: String) -> String {
    z // z is returned and moves out to the calling function
}
```

reasons ownership is important
- ownership prevents memory safety issues:
  - dangling pointers(points to nowhere or rubbish data)
  - double free(trying to free memory that has already been freed)
  - memory leaks(not freeing memory that should have been freed)

#
## Borrowing
this is a way of temporarily accessing 
data(immutabily or mutably) without taking ownership of it 
when this is done you're taking a reference(pointer)
to the data, not the data itself, this prevents dangling 
pointers and data races. just like ownership it has it's own 
set of rules that must be complied with otherwise the program 
won't compile

**Rules of reference:**
1. at any given time, you can have either one mutable
reference or any number of immutable references
2. references must always be valid

to reference a variable use the `&` synbol 

Eg:  
immutable reference
```rust
fn main(){
    let s1: String = String::from("hello");

    // a reference of "s1" is passed to the calculate_length()
    let len = calculate_length(&s1);

    // note how s1 is still usable on this line
    println!("the length of '{}' is {}.", s1, len);
}

// this function accepts a reference of a string as a param
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

mutable reference
```rust
fn main(){
    // mutable variable declaration
    let mut s = String::from("hello");

    // pass in a mutable reference
    // note the usage of the "mut" keyword here
    // it is very important that it is explicitly included 
    change(&mut s);
}

fn change(x: &mut String){
    x.push_str(", world");
}
```

### example of the rules of borrowing
```rust
// mutable variable declaration
let mut s = String::from("hello");


let r1 = &mut s;
// the statement below violates the 1 rule of borrowing
// since "r1" already made an mutable reference to s
let r2 = &mut s; // error
```

```rust
// mutable variable declaration
let mut s = String::from("hello");

{
    let r1 = &mut s; // valid within this inner scope
}// "r1" goes out of scope here

// the operation below is valid because the first mutable
// reference was not done in the same scope as "r2"
let r2 = &mut s; // valid within this outside scope
```
```rust
// mutable variable declaration
let mut s = String::from("hello");

let r1 = &s; // vaid immutable reference
let r2 = &s; // vaid immutable reference
let r3 = &mut s; // violates first rule

println!("{}, {}, and {}", r1, r2, r3);
```
```rust
// mutable variable declaration
let mut s = String::from("hello");

let r1 = &s; // vaid immutable reference
let r2 = &s; // vaid immutable reference

println!("{} and {}", r1, r2);
// after this point "r1" and "r2" won't be used anymore


let r3 = &mut s; // valid mutable reference
println!("{}", r3);
```
```rust
fn main(){
    // "ref_to_nothing" is being assigned a reference 
    // that has gone out of scope(a garbage value)
    let ref_to_nothing = dangle();
}

fn dangle(){
    let s = String::from("hello");

    // return a reference to "s"
    &s // this violates the second rule (to fix it just return "s" not a reference)
}// because "s" goes out of scope here
```

2:22:00

```rust
let mut x Box<value type> = Box::new(value); // this stores whatever value passed to it into the heap memory

*x = another_value // the "*" is used for de-refrencing
// bascically retrieving/accessing the actual value and not the pointer
// and if you want to directly modify the value at that memory address that the variable is pointing to 
```