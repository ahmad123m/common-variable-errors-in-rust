fn main() {
    // Here if we don't use *mut* the code will show an error
    let mut x = 5; // ❌ Without mut, reassignment fails
    x = 10; // ✅ Fixed with mut
    println!("x is {}", x);
}
