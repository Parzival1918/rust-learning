fn main() {
    /* This code does not compile because we are changing 
     * an immutable variable.
    let x = 5;
    println!("x is: {x}");
    x = 6;
    println!("x is: {x}");
    */

    // This will compile because we set the variable to mutable
    let mut x = 5;
    println!("x is: {x}");
    x = 6;
    println!("x is: {x}");

    // Constant
    const ONE_HOUR_IN_MINS: u32 = 60;
    const THREE_HOURS_IN_MINS: u32 = 3 * ONE_HOUR_IN_MINS;
    println!("A const: {THREE_HOURS_IN_MINS}");
}
