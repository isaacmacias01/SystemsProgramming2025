fn assignment1(){

    fn fahrenheit_to_celsius(f: f64) -> f64 {
        (f - 32.0) * (5.0 / 9.0)
    }

    fn celsius_to_fahrenheit(c: f64) -> f64 {
        (c * (9.0/5.0)) + 32.0
    }

    let name = "Isaac Macias";
    let course = "CSCI-3334-01";
    println!("Module 1-3");
    println!("{}",name);
    println!("{}",course);
    println!();

    const WATER_FREEZE: i32 = 32; // Const for the freezing point of water in Fahrenheit
    let mut fahernheit: f64 = 97.0; // Mutable variable with temp in Fahrenheit

    let in_celsius = fahrenheit_to_celsius(fahernheit); // Fahernheit -> celsius
    println!("{:.1}°F -> {:.1}°C", fahernheit, in_celsius); // Print conversion

    let mut counter = 0;
    while counter < 5 {
        fahernheit += 1.0;
        let c = fahrenheit_to_celsius(fahernheit);
        println!("{:.1}°F -> {:.1}°C", fahernheit, c);
        counter += 1;
    }

}

fn assignment2(){

    let numbers: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; // Array numbers with 10 elements

    fn is_even(n: i32) -> bool {
        n % 2 == 0 // returns true if n is even
    }

    for idx in 0..numbers.len() {
        println!();
        
        let num = numbers[idx];

        // Check even or odd
        if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }

        // Check FizzBuzz
        if num % 3 == 0 && num % 5 == 0 {
            println!("Fizz and Buzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        }

        println!(""); // Empty line for readability
    }

     // WHILE LOOP — Just compute the total sum
    let mut counter = numbers.len(); // Start at 10
    let mut index = 0;
    let mut sum = 0;

    while counter != 0 {
        sum += numbers[index];
        counter -= 1;
        index += 1;
    }

    println!("The total sum of the array is: {}", sum);

    // FOR LOOP TO FIND THE LARGEST NUMBER (using your preferred style)
    let mut largest = numbers[0];

    for idx in 0..numbers.len() {
        if numbers[idx] > largest {
            largest = numbers[idx];
        }
    }

    println!("The largest number in the array is: {}", largest);
}

// For assignment3 (guessing game)
fn chk(g: i32, s: i32) -> i32 {
    if g == s {
        0
    } else if g > s {
        1
    } else {
        -1
    }
}

fn assignment3(){
    println!();

    let s_num: i32 = 7;
    let gs = [10, 12, 5, 7];
    let mut c = 0;

    for idx in 0..gs.len() {
        c += 1;
        let r = chk(gs[idx], s_num);

        if r == 0 {
            println!("Guess {} is correct!", gs[idx]);
            break;
        } else if r == 1 {
            println!("Guess {} is too high.", gs[idx]);
        } else {
            println!("Guess {} is too low.", gs[idx]);
        }
    }

    println!("It took {} guesses.", c);
}

fn main(){
    assignment1();
    assignment2();
    assignment3();
}