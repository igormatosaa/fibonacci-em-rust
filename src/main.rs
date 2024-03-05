use std::io;

fn main() {
    let mut x:i32 = 0;
    let mut y:i32 = 1;

    let mut fibo = String::new();

    io::stdin()
        .read_line(&mut fibo)
        .expect("Failed to read line");

    let mut fibo: usize = fibo
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let mut result:i32 = x;

    if fibo == 1 { 
        result = 0; 
    } else if fibo == 2 { 
        result = y; 
    } else {
        while fibo >= 3 {
            result = fibonacci(x, y);
            x=y;
            y=result;

            fibo = fibo - 1;
        }
    }

    println!("{result}");

}

fn fibonacci(x:i32 , y:i32) -> i32{
    x+y
}