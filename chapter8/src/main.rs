use std::collections::HashMap;

// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
fn average_of (list: &Vec<usize>) -> f32 {
    let mut total = 0;
    for i in list.iter() {
        total += i
    }
    let total = total as f32;
    total / (list.len() as f32)
}

fn median_of (list: &Vec<usize>) -> usize {
    let mut sorted: Vec<usize> = Vec::new();
    for i in 0..list.len() - 1 {
        sorted.push(list[i]);
    }
    for i in 0..sorted.len() {
        for j in (i+1)..sorted.len() {
            if sorted[i] > sorted[j] {
                let temp = sorted[i];
                sorted[i] = sorted[j];
                sorted[j] = temp;
            }
        }
    }
    sorted[sorted.len() / 2]
}

fn mode_of (list: &Vec<usize>) -> Vec<usize> {
    let mut counter: HashMap<usize, usize> = HashMap::new();
    for i in list.iter() {
        let count = counter.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut mode: Vec<usize> = Vec::new();
    let mut max_count: usize = 0;
    for (num, count) in counter.iter() {
        if count > &max_count {
            max_count = *count;
            mode = vec!(*num);
        } else if count == &max_count {
            mode.push(*num);
        }
    }
    mode
}


// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn pig_latin(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }
    let vowel :[char; 5] = ['a', 'o', 'e', 'i', 'u'];
    let s = String::from(word);
    let chars = s.char_indices();
    let mut new_chars: Vec<char> = Vec::new();
    let mut is_start_with_vowel = false;
    let mut first_char: char = '0';
    for (i,  c) in chars {
        if i == 0 {
            if vowel.contains(&c) {
                is_start_with_vowel = true;
                new_chars.push(c);
            }
            first_char = c;
        } else {
            new_chars.push(c);
        }
    }
    let suffix = if is_start_with_vowel {
        String::from("-hay")
    } else {
        "-" .to_owned()+ &first_char.to_string() + "ay"
    };
    let mut result = String::new();
    for c in new_chars {
        result.push(c);
    }
    result + &suffix
}


// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.



fn main() {
    let a_list = vec![2,0,2,1,1,1,1,4,2,2,3,3];
    println!("We have a list with {:?}", a_list);
    println!("It's average: {}", average_of(&a_list));
    println!("It's median: {}", median_of(&a_list));
    println!("It's mode: {:?}", mode_of(&a_list));

    println!("Apple's Pig Latin is {}", pig_latin("apple"));
    println!("World's Pig Latin is {}", pig_latin("world"));
    println!("empty Pig Latin is {}", pig_latin(""));
    println!("H's Pig Latin is {}", pig_latin("h"));
}
