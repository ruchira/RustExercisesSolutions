use std::io;

fn main() {

    let n = get_positive_number();

    let fib = fibonacci(n);

    println!("Fibonacci number {} is {}", n, fib);
}

fn fibonacci(n : u32) -> u32 {
    if n == 1 {
        return 1
    } else if n == 2 {
        return 1
    } else {
        let mut i0 = 1;
        let mut i1 = 1;
        let mut sum = 2;
        let mut counter = 2;
        while counter < n {
            sum = i0 + i1;
            i0 = i1;
            i1 = sum;
            counter += 1;
        }
        return sum;
    }
}

fn get_positive_number() -> u32 {
    loop {
        println!("Please enter a positive number.");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => 
                if num > 0 {
                    return num
                } else {
                    println!("The number must be greater than zero.");
                    continue
                },
            Err(_) => continue,
        };
    }
}
