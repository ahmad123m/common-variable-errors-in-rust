fn main() {
    let mut x = 5;
    let r1 = &mut x;
    // let r2 = &mut x; // ❌ Cannot borrow mutably twice
    println!("r1 points to {}", r1);
}
