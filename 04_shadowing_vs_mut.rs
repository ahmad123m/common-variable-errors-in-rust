fn main() {
    // The core concept: In let we used let two times so the second let will hide the first let condition
    // But in mut we made the variable mutable, so from now you can use y without using let or shadowing
    let x = 5;
    let x = x + 2; // ✅ Shadowing
    println!("Shadowed x: {}", x);

    let mut y = 5;
    y = y + 2; // ✅ Mutable variable
    println!("Mutable y: {}", y);
}
