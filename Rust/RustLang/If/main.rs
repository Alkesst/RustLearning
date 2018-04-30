fn main(){
    let x = 5;
    if x == 5 {
        println!("x value is {}", x);
    } else {
        println!("x is not five :(");
    }
    // Also you can do this
    let y = if x == 5 {
        10
    } else {
        15
    };
    // More compact. Same behaviour
    let y = if x == 5 { 10 } else { 15 }; // y: i32
}