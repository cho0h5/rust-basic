fn main() {
    for n in 1..11 {
        println!("fib({}): {}", n, fib(n));
    }
    println!("fib({}): {}", 92, fib(92));
}

fn fib(n: u64) -> u64 {
    if n <= 2 {
        return 1;
    }

    let mut x1 = 1;
    let mut x2 = 1;

    for _ in 1..n {
        let temp = x2;
        x2 = x1 + x2;
        x1 = temp;
    }
    
    x1
}
