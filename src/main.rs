fn main() {
    println!("Welcome to the Fizz Buzz program!");
    fizz_buzz();
}

fn fizz_buzz() {
    let mut fizz_buzz_count = 0;

    for i in 1..=300 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizz buzz");
            fizz_buzz_count += 1;
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }

    println!("'fizz buzz' occurred {} times.", fizz_buzz_count);
}