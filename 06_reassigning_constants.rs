// Once you have assigned a constant, you cannot assign anything new to it again
const NUMBER: i32 = 3;
fn main() {
    // NUMBER = 5; // ❌ Cannot reassign a constant
    println!("Number: {NUMBER}");
}
