
/*
 * 
 * 
 */
fn main(){
    immutableVariable();
    mutableVariable();
}

/*
 * In Rust, variables are immutable only by default.
 */
fn immutableVariable(){
    let x = 0;
    // x = 1; //cannot assign twice to immutable variable `x`
    println!("x = {}", x);
}

/*
 * Sometimes mutablility can be very useful.
 * However, we still have the option to make variables mutable.
 * 
 * We can make variables mutable by adding mut in front of the variable name.
 */
fn mutableVariable(){
    let mut x = 0;
    // x = "Bert"; // expected integer, found `&str`
    x = 100;
    println!("x = {}", x);
}