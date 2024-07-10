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
    println!("Recurssion {}", fact(3));

    // Control FLow

    let x = 6;
    if x < 5 {
        println!("Smaller");

    } else {
        println!("Bigger");
    }

    // Conditional statements
    let y : char = if x < 5 {'s'} else {'b'};
    println!("{}", y);

    // loop with results
    let mut cnt = 0;
    let  result = loop {
        cnt += 1;
        if cnt == 5 {
            break cnt * 2;
        }
    };
    println!("Result {}", result);

    // Loop Labels
    cnt = 0;
    'test_loop: loop {
        println!("Count : {cnt}");
       let mut remaining = 10;
        loop {
            println!("Remaining: {remaining}");
            if remaining == 1 {
                break 'test_loop
            }
            remaining -= 1;
        }
        cnt += 2;
    }

    // while

    cnt = 0;
    while cnt < 10  {
        println!( "While {cnt}");
        cnt += 1;
    }

    // for works like a foreach
    for element in arr {
        println!("For {element}");
    }
    // For range
    for number in (1..4).rev() {
        println!("For Range {number}");
    }
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