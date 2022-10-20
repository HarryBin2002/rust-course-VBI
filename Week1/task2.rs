use std::io;

fn main() {

    // input the sentence includes the char.
    let sentence = input_data();
    // input the char needed to find and count.
    let word = input_data();

    
    // 
    count_chars(&sentence, &word);
    
}

/**
 * This function to input data from keyboard.
 */
pub fn input_data() -> String {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("failed to readline");
    
    return input;
}

/**
 * This function to excute logic code:
 * loop the string and checking the char appear or not
 */
pub fn count_chars(sentence: &String, word: &String)  {
    // create a variable to store the number.
    let mut count: u32 = 0;

    // loop all chars in sentence.
    for s in sentence.chars() {
        if s == word.as_bytes()[0] as char {
            // if the char appear, count add 1
            count += 1;
        }
    }
    
    println!("The number of occurrences of this char is: {}", count);
}


