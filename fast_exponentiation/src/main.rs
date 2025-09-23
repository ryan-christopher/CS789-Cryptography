// Lecture 3 Assignment - write a program for the fast exponentiation algorithm

// fast_exponentation takes i64 values for x, e, and n
// and calculates (x**e) mod n, returns value as y
fn fast_exponentiation(mut x: i64, mut e: i64, n: i64) -> i64 {
    // set y to default value of 1
    let mut y = 1;
    // get base value to raise x to the power of
    let base = x as u32;
    // continue until value is 0
    while e > 0 {
        println!("x: {:?}, e: {:?}, y: {:?}", x, e, y);
        // if e is even, the ending binary digit is 0, divide e
        // by 2 and raise x by power of base
        if e % 2 == 0 {
            e = e / 2;
            x = x.pow(base) % n;
        }
        // if e is odd, subtract 1 from e, multiply y by x,
        // and get the product mod n
        else {
            e -= 1;
            y = (x * y) % n;
        }
    }
    // return y when done
    y
}

fn main() {
    println!("{:?}", fast_exponentiation(2, 100, 71));
}
