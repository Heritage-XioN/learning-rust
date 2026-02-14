// the compiler can be given specific commands eg:
// #[allow(unused_variables)]



// find out what this -> :: means
// for getting values within a range use
// ".." eg: 3..10 or 3..=10 
// the "=" means that the last value is included in the range

// main entry point
fn main() {
    // make sure all statements end with ";"
    // also rust is a bueaty in the sense that the compiler 
    // will almost always tell you what the exact issue is
    // and how to fix/resolve it

    // Data Types
    // check out the readme

    // calling a declared function
    learning_variables();
}

// function declaration just like the main
fn learning_variables(){
    println!("learing varibles starts here!!!!");
    // declare a variable
    let a: i32 = 23;

    // to salfely create an unitialized variable or
    // an initialized but unused variable
    // prefix it with this "_"
    let _b: i32;

    // in rust all variables are immutable by default
    // meaning they cant be changed ones declared
    // the way to handle this is by using "mut"
    // to mark it as mutable
    let mut c: i32 = 5;
    c += 5;

    // shadowing concept: this entails that a declared variable 
    // can be re-declared and used somewhere else in the code 
    // without causing any issues. but i generally dont support using that
    // so if you want to learn about it then hit up the official docs

    // variables can also be declared using destructuring 
    // in this case we destructured a tuple
    // remember adding "mut" makes it mutable
    let ( mut d, e) = (2, 4);
    d += 8;

    // varibales can be created using destructuring assignments
    let (f, g);
    // using this ".." in the destructuring assignment we can tell the compiler to just give us the
    // value at the coresponding position as with our declared variable in this case f or g
    // also note the data structure when destructuring
    (f, ..) = (25, 75); // tuple 
    [.., g] = [65, 21]; // array

    // the value that is assigned to a variable can be type annoted
    // like so value_dataType also note that the underscore is not necessary
    // and is only used for readability sake
    let h: u8 = 8_u8; // in this case 8 is the value u8 is the dataType

    // a variable can only be assigned a value with matching dataType 
    // if the value you wish to assign it is not of the same dataType 
    // you can use "as" followed by the variables dataType the compiler
    // will convert the value to the specified data type if possible
    let i: u16 = 4_u8 as u16;

    let _j: String = "hello world".to_string();
    let _s = String::from("hello, world");

    // this is the equality macro
    // used to compare if two values are eqaul to each other
    assert_eq!(c, 10);

    //print!("value of x: {x}");
    // both println macro below do the same thing
    println!("value of d: {d}. value of e: {e}");
    println!("value of a: {}. value of c: {}", a, c);
    println!("value of f: {}. value of g: {}", f, g);
    println!("value of h: {}. value of i: {}", h, i);

    println!("learing varibles ends here!!!!");
}