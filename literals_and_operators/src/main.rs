fn main() {
    //integere addition
    println!("1 + 2 = {}", 1u32 + 2);
    //integer subtraction
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);
    // short circuiting boolean
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    //bitwise operators
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("1 << 5 = {}", 1u32 << 5);

    //use underscore to improve readability
    println!("One billion is written as {}", 1_000_000_000u32)
}
