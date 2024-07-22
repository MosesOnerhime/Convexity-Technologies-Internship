fn array_traverser(n: i32) {
    if n <= 0 {
        println!("'n' should be a positive integer.");
        return;
    }

    let mut array: Vec<i32> = Vec::new();
    for i in 1..=n {
        array.push(i);
    }

    // Loop through array
    for value in &array {
        println!("{}", value);
    }
}


fn main() {
    
    let n = 10;
    array_traverser(n);
}
