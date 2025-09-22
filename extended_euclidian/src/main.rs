// Lecture 1, Assignment 3 - write code that will find a couple integers, x and y, for
// given integers m and n, such that xm + yn will yield the smallest positive number
fn extended_euclidian(val_1: i64, val_2: i64, mut val_list: Vec<i64>) {
    let m;
    let n;
    // determine larger value
    if val_1 > val_2 {
        m = val_1;
        n = val_2;
    } else {
        m = val_2;
        n = val_1;
    }
    // calculate modulo
    let z = m % n;
    // recursively call the function until the value of z is 0 or 1
    // indicating that the gcd has been found
    if z == 0 {
        //println!("m: {:?}, n: {:?}, z: {:?}", m, n, z);
        val_list.push(m);
        println!("gcd: {:?}", n);
        find_coefficients(
            1,
            val_list[val_list.len() - 2],
            -1,
            val_list[val_list.len() - 1],
            n,
            val_list,
        );
    } else if z == 1 {
        //println!("m: {:?}, n: {:?}, z: {:?}", m, n, z);
        val_list.push(m);
        val_list.push(1);
        println!("gcd: {:?}", z);
        find_coefficients(1, m, -1, n, z, val_list);
    } else {
        //println!("m: {:?}, n: {:?}, z: {:?}", m, n, z);
        val_list.push(m);
        extended_euclidian(n, z, val_list);
    }
}

// pass gcd, starting values, and list of divisors to work up to the original
// largest value
fn find_coefficients(
    mut x: i64,
    mut m: i64,
    mut y: i64,
    mut n: i64,
    gcd: i64,
    mut val_list: Vec<i64>,
) {
    // begin replacing factors on n first, alternate between
    // n and m
    let mut n_turn = true;
    let initial_val = val_list[0];
    // continue until the original largest value is reached
    while m != initial_val && n != initial_val {
        // when n is being repalced, remove 2 values from the list
        if n_turn {
            n = val_list[val_list.len() - 3];
            val_list.pop();
            val_list.pop();
            // update x val
            x = x + (y * -1) * (n / m);
        } else {
            // when m is being replaced, don't remove any values
            m = val_list[val_list.len() - 2];
            // update y val
            y = y - x * (m / n);
        }
        n_turn = !n_turn;
    }
    println!("{:?}({:?}) + {:?}({:?}) = {:?}", x, m, y, n, gcd);
    println!("x = {:?}", x);
    println!("y = {:?}", y);
}

fn main() {
    println!("\nExtended Euclidian (513, 614)");
    extended_euclidian(513, 614, Vec::<i64>::new());
    println!("\nExtended Euclidian (1024, 888)");
    extended_euclidian(1024, 888, Vec::<i64>::new());
    //println!("\nExtended Euclidian (8562152543, 893884534109213)");
    //extended_euclidian(8562152543, 893884534109213, Vec::<i64>::new());
}
