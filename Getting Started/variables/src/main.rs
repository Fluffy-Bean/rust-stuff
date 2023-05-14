fn main() {
    // Variables are immutable (unchangeable) by default
    // u is a unsigned integer, the number is defining the size of the integer in bits

    let x = 9; // x is a u32 thats assigned at compile time
    let y: u32 = 10; // y is a u32 thats strongly typed

    let _z: u32 = 10; // z is a u64 thats strongly typed, but not used so it starts with an underscore

    let mut h: u32 = 19; // mut makes the variable mutable, meaning it can be changed
    h = 21; // h is now 21

    println!("{} plus {} is {}", x, y, h); // println (print line) is a macro so it has a ! at the end
                                           // {} are part of a formatted string and are replaced by the variables in the order they are given
}
