fn main() {
    println!("Hello world!");
    let a = [1, 2, 3];
    let mut _b = [4, 5, 6];
    println!("{:?}", a);
    println!("{:?}", _b);
    println!("{:#?}", a);
    _b[1] = 2;
    println!("{:?}", _b);
    let tuple = (1, 2, "Hello", "world ;)");
    println!("{:#?}", tuple);
    let b: (i32, f64) = (1, 1.5);
    println!("{:?}", b);
    let (e, _, _, f) = tuple;
    println!("{:?}", e);
    println!("{:?}", f);
    /* Slice things :) */
    let aa: [i32; 4] = [1, 2, 3, 4];
    let bb: &[i32] = &aa;
    println!("{:?}", aa);
    println!("{:?}", bb);
    println!("{:?}", &aa[0..2]);
    println!("My number = {}", plus_one(1));
}

fn plus_one(a: i32) -> i32{
    a + 1
}