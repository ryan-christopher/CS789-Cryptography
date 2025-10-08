// Lecture 4 - write a program for the baby-step giant-step algorithm

// baby_step_giant_step takes the variables log_base, log_val, and z
// as input and finds the discrete log (the value to raise the log_base
// to in order to get log_val mod z)
fn baby_step_giant_step(log_base: i64, log_val: i64, z: i64) {
    // find m by taking the cieling of the square root of z-1
    let mut m = (z as f64).sqrt().ceil() as i64;

    // create vector to store list of values for j, then
    // calculate each value for j from 0 to m-1 as (log_base^j) mod z
    let mut j_list = Vec::<i64>::new();
    for j in 0..m {
        j_list.push(log_base.pow(j as u32) % z);
    }

    // create vector to store list of values for i
    // [giving up here, has something to do with multiplicative inverse, use extended euclidian algo and if it is 1 do something?]
    println!("m: {}", m);
    println!("j: {:?}", j_list);
    println!("Baby steps and giant steps");
}

fn main() {
    baby_step_giant_step(2, 3, 29);
}
