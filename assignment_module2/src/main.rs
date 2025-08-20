fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    let mut sum = 0;
    let mut i = low;

    while i <= high {
        sum += i;
        i += step;
    }

    *total = sum;
}

fn most_frequent_word(text: &str) -> (String, usize) {
    let mut counts: Vec<(String, usize)> = Vec::new();

    for word in text.split_whitespace() {
        // Check if the word already exists in counts
        let mut found = false;
        for (w, c) in counts.iter_mut() {
            if w == word {
                *c += 1;
                found = true;
                break;
            }
        }
        // If not found, push new entry
        if !found {
            counts.push((word.to_string(), 1));
        }
    }

    // Now find the word with max count
    let mut max_word = String::new();
    let mut max_count = 0;
    for (w, c) in &counts {
        if *c > max_count {
            max_word = w.clone();
            max_count = *c;
        }
    }

    (max_word, max_count)
}

fn main() {

    println!("MODULE 2 : ASSIGNMENT 1");
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);
    println!();


    println!("MODULE 2 : ASSIGNMENT 2");
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
