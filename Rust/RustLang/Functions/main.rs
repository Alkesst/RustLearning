fn main(){
    print_number(5);
    foo();
    print_sum(2, 5);
    println!("value of 2 + 1: {}", add_one(2));
}

fn foo(){
}

fn print_number(x: i32){
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32){
    println!("x + y = {}", x + y);
}

fn add_one(x: i32) -> i32{
    //if we put a semicolon on x + 1 the compiler won't compile!!!
    x + 1
}

// Expressions vs. Statements