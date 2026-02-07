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
dynamic strings scalar UTF-8 encoded that are heap allocated, mutable and are owned
| length  | type    |         use        |
|---------|---------|--------------------|
| dynamic | String  |  " ".to_string()   |

### string - str
dynamic strings scalar UTF-8 encoded that are a borrowed immutable reference
| length | type    |   use  |
|--------|---------|--------|
| 4-bytes |  &str  |   " "  |

### unit type
this is an empty tuple of size/length 0 used to return nothing in expressions or functions
| length | type    |
|--------|---------|
|    0   |   ()    |

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

*Rules of ownership:*
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
    let x = "Hello" // this is only valid within the "{}"

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
lost. there a two main regions in a RAM used by a program at runtime they are the **stack** and **heap**
memoru 