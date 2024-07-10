use std::cmp::Ordering;

fn main() {
    // Int Data Types
    let x : u32 = 250;
    println!("{}", x);
    // Floats
    let x : f32 = 2.0;
    println!("{}", x);

    // Char
    let x : char = 'z';
    println!("{}", x);

    // Boolean
    let x : bool = true;

    // Compound types
    // Tuples

    let tup : (u32, f32, bool) = (12, 2.0, false);
    // Destructing tuple
    let (x, y, z) = tup;
    println!("{},{}, {}", x, y, z);
    // Accessing tuple values
    println!("{},{}, {}", tup.0, tup.1, tup.2);

    // Array (all values are the same type)
    let arr : [u32; 3] = [1, 2, 3];
    println!("{},{}, {}", arr[0], arr[1], arr[2]);

    // Functions
    println!("Functions : {}", sum_fn(2, 3));

    // Statements and expressions
    let x : i32 = {
        let y = 32;
        y + y
    };
    println!("{}", y);
    // Recursion
    println!("Recurssion {}", fact(3))
}

fn sum_fn(x : i32, y : i32) -> i32{
     x + y // no return needed
}
fn fact(n : i32) -> i32 {
    match n.cmp(&0) {
        Ordering::Equal => 1,
        Ordering::Greater => n * fact(n - 1 ),
        Ordering::Less => -1
    }
}