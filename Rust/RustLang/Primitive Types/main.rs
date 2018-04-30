fn main(){
    // Boolean types
    let x = true;
    let y: bool = false;

    //Character types
    let x = 'x';
    let two_hearts = '💕';
    println!("{} {}", two_hearts, x);

    //Numeric Types
    let x = 42; // `x` has type `i32`.
    let y = 1.0; // `y` has type `f64`.

    /*
     * List of numeric types
     * i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64
     */

    // unsigned types begin with u, signed with i.

    //Arrays
    let a = [1, 2, 3]; // a: [i32; 3]
    let mut m = [1, 2, 3]; // m: [i32; 3]

    // initializes an array with the assigned value
    let b = [0; 20]; // b is an array with 20 elements which values are 0
    println!("a has {} elements", a.len());

    let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]
    println!("The second name is: {}", names[1]);

    /*
     * SLICES
    */

    // Syntax
    let a = [0, 1, 2, 3, 4];
    let complete = &a[..]; //Array with whole elements
    let middle = &a[1..4]; //Array with the elements from 1 to 4, not included.

    // TUPLES

    let tup = (1, "Hello");
    // Tuples are not mutable by definition.
    // Anyway You can assign one tuple into another, if they have the same contained types and arity.
    // Arity refers to the number of arguments a function or operation takes.
    let mut x: (i32, &str) = (1, "hello");
    let y = (2, "Bye");
    x = y;
    println!("{:?}", x);
    // TUPLES OF SINGLE ELEMENTS
    let single_tup = (0,); // important to put , because (0) is just a 0 with parentheses.

    // TUPLE INDEXING
    let tuple = (1, 2, 3);
    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;
    println!("x is {}", x);
    // Functions have type too!
    // x is a ‘function pointer’ to a function that takes an i32 and returns an i32.
    fn foo(x: i32) -> i32 { x }
    let x: fn(i32) -> i32 = foo;
    println!("{:?}", x);

}