// Lecture 1, Assignment 2 - write code for the Euclidean Algorithm
fn euclid(val_1: i64, val_2: i64) {
    let x;
    let y;
    // determine larger value
    if val_1 > val_2 {
        x = val_1;
        y = val_2;
    } else {
        x = val_2;
        y = val_1;
    }
    // calculate modulo
    let z = x % y;
    // recursively call the function until the value of z is 0 or 1
    // indicating that the gcd has been found
    if z == 0 {
        println!("m: {:?}, n: {:?}, z: {:?}", x, y, z);
        println!("gcd: {:?}", y);
    } else if z == 1 {
        println!("m: {:?}, n: {:?}, z: {:?}", x, y, z);
        println!("gcd: {:?}", z);
    } else {
        println!("m: {:?}, n: {:?}, z: {:?}", x, y, z);
        euclid(y, z);
    }
}

fn main() {
    //euclid(513, 614);
    euclid(1024, 888);
    //euclid(8562152543, 893884534109213);
}
