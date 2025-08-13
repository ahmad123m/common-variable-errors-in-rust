fn main() {
    let x = "3";
    // println!("{}", x + 2); // ❌ Cannot add &str and i32


    // ::<i32> is a turbofish syntax haha - a new word
    let x = x.parse::<i32>().unwrap(); // ✅ Convert to integer
    println!("{}", x + 2);
}
