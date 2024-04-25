fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    n * factorial(n - 1)
}

fn main() {
    let n = 5;
    println!("Factorial of {} is {}", n, factorial(n));
}
