fn main() {
    println!("Iterating fibonacci of 10: {}", iterative_fibonacci(10));
}

fn iterative_fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else {
        let mut x = 0;
        let mut y = 1;

        println!("1: {}", x);

        for i in 1..n-1 {
            println!("{}: {}", i+1, y);

            let z = x + y;
            x = y;
            y = z;
        }

        return y;
    };

}