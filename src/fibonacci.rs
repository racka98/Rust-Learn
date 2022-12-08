pub fn fibonacci_num(n: isize) {
    let mut count = 0;
    let mut num1 = 0;
    let mut num2 = 1;
    while count < n {
        print!("{num1} + ");
        let sum = num1 + num2;
        num1 = num2;
        num2 = sum;
        count += 1;
    }
    println!("\n");
}
