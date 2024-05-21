use std::io;
fn main() {
    println!("which fibonacci number do you want to generate?");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Something went wrong");

    let index: u32 = index.trim().parse().expect("Please type a number!");
    let fib_num: u32 = fib(index);
    println!("fib_num is: {}", fib_num);
    fn fib(n: u32) -> u32 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else {
            return fib(n - 1) + fib(n - 2);
        }
    }
}
