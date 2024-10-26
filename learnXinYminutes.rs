// This is a comment. Line comment look like this...
// and extend multiple lines lik this.

/* Block Comments
/* can be nested. */ */

/// Documentation comments look like this and support markdwon notation.
/// # Examples
///
/// ```
/// let five = 5
/// ```

// 1. Basics//

#[allow(dead_code)]

// Functions
//`i32` is the type for 32-bit signed integers
fn add2(x: i32, y: i32) -> i32{
    //Implicit return (no semicolon)
    x+y
}


#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // Numbers //

    // Immutable Bindings //
    let x: i32 = 1;

    // Integer/Float Suffixes //
    let y: i32 = 13i32;
    let f: f64 = 1.3f64;


    // Type Inference
    // Most of the time, the Rust compiler can infer what type of variable is, so
    // you don't have to write an explicit type annotation
    // Throughout this tutorial, types are explicitly annotated in many places,
    // but only for demonstrative purposes. Type inference can handle this for
    // you most of the time.

    let implicit_x = 1;
    let implicit_f = 1.3;

    // Arithmetic
    let sum = x + y+ 13;

    // Mutable Variable
    let mut mutable = 1;
    mutable = 4;
    mutable += 2;

    // Strings //

    // String Literals //
    let x: &str = "Hello Rusty";

    // Printing
    println!("{} {}", f, x);// 1.3 Hello Rusty

    // A `String` - a heap-allocated String
    // Stored as a `Vec<u8>` and always holds a valid UTF-8 Sequence.
    // which is not null terminated.
    let s: String = "hello world!".to_string();
    
    // A string slice - an immutable view into another string
    // This is a reference to a string
    // doesn't actually contain the contents of the string, just a pointer to 
    // the beginning and a lenght of a string buffer.
    // Statically allocated or contained in another object (in this case `s`).
    // The string slice is like a view `&[u8]` into a `Vec<T>` .
    
    let s_slice: &str = &s; 
    // declaring the variable s_slice of type &str string literal
    // and assigning it to the reference of String s. 
    

}

