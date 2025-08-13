// Shadowing

fn main() {
    let x = "hello";
    // x = 5; // ❌ Cannot change type

    // Basically in Shadowing we use the keyword like *let* so now the Rust will not show the previous value of x in code

    let x = 5; // ✅ Shadowing allows type change
    println!("x is {}", x);
}
