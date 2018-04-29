fn main(){
    // x is not a variable. We bind a value into a name.
    let x = 5;
    // Patterns
    let (x,y) = (1, 2);
    println!("{}, {}", x, y);
    let a: i32 = 5; // 'a' is a binding with the type i32 (32-bit signed integer) and the value 5
    // Mutability
    let mut b = 5;
    // Initializing Bindings
    println!("{}", b);
    b = 10;
    println!("{}", b);
    /* If we do the next instructions
     * let x: i32;
     * println!("The value of x is {}", x);
     * won't compile, because of x it is uninitialized. 
     */
    //Scopes
    let j: i32 = 43;
    {
        let i: i32 = 32;
        println!("Here we can see i ({}) and j ({})", i, j);
    }
    println!("Here we can't see i, but j yes ({})", j); // if we put after i ({}) and then ", i" the compiler will show a warning
    //Shadowing
    let x: i32 = 8;
    {
        println!("{}", x); // Prints "8".
        let x = 12;
        println!("{}", x); // Prints "12".
    }
    println!("{}", x); // Prints "8".
    let x =  42;
    println!("{}", x); // Prints "42".
}