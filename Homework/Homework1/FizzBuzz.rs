fn main() {
    for i in 1u32..=100 {

        let div_3 = i % 3 == 0;
        let div_5 = i % 5 == 0;

        if div_3 & div_5 {
            println!("FizzBuzz");
        } else if div_3 {
            println!("Fizz");
        } else if div_5 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}
