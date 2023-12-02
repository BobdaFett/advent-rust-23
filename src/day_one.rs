// Calibration value can be found by combining the first and last digit for a single TWO DIGIT
// NUMBER.
// Each line has at least one digit, but not always two.
// The end goal is to find the sum of all the calibration values.
//
// This algorithm isn't necessarily the most efficient, but it'll work.

// In the second version of this program, we also need to find all the strings that contain
// the word form of the calibration values. For example, we need to be able to handle the string
// "one" and convert it to the number 1, and so on.

pub fn run() -> std::io::Result<()> {
    let mut sum = 0;
    // Read information from a file. This file should be stored in your user documents.
    let file = std::fs::read_to_string("C:\\Users\\lvas4\\Documents\\advent-rust-23\\day_one\\input.txt")
        .expect("Unable to read file.");
    
    for line in file.lines() {
        let mut line_digits: Vec<i32> = Vec::new();

        let line = line.to_string();

        let chars = line.chars().collect::<Vec<char>>();

        // Iterate through the vector of characters.
        // At first we can start with iterating towards the end from the beginning.
        for (i, char) in chars.iter().enumerate() {
            // Check if the char is a number.
            if let Some(num) = is_number(&char) {
                line_digits.push(num);
                break;
            }

            // Check if the char is a number in word form.
            // If it is, we need to convert it to a number.
            if let Some(num) = check_word_number(&line, &char, i) {
                line_digits.push(num);
                break;
            }
        }
        
        // Now iterate the opposite direction.
        for (i, char) in chars.iter().rev().enumerate() {
            // Check if the char is a number.
            if let Some(num) = is_number(&char) {
                line_digits.push(num);
                break;
            }

            // Check if the char is a number in word form.
            if let Some(num) = check_word_number(&line, &char, chars.len() - i - 1) {
                line_digits.push(num);
                break;
            }
        }

        // Get the value of the line sum.
        let line_sum = (line_digits[0] * 10) + line_digits[1];
        // println!("Line {}: {:?}", line, line_sum);

        // Add the line sum to the total sum.
        sum += line_sum;
    }

    // Print the total sum.
    println!("The total sum of our calibration values is: {:?}", sum);

    Ok(())

}

fn is_number(string: &char) -> Option<i32> {
    match string.is_ascii_digit() {
        true => Some(string.to_digit(10).unwrap() as i32),
        false => None,
    }
}

fn check_word_number(full_string: &String, char: &char, index: usize) -> Option<i32> {
    // Check if the char passed could be the first letter of a number in word form.
    // If it is, we need to check the next few characters to see if they match the rest of the
    // word.
    // I genuinely dislike this function, but I can't think of a better way to do it.

    let mut result: Option<i32> = None;
    let check_slice = full_string.chars().skip(index);

    match char {
        'o' => {
            if check_slice.clone().count() >= 3 {
                let check_string = check_slice.take(3).collect::<String>();
                if check_string.eq("one") {
                    result = Some(1);
                }
            }
        },
        't' => {
            if check_slice.clone().count() >= 3 {
                let check_string = check_slice.clone().take(3).collect::<String>();
                if check_string.eq("two") {
                    result = Some(2);
                }
                if check_slice.clone().count() >= 5 {
                    let check_string = check_slice.take(5).collect::<String>();
                    if check_string.eq("three") {
                        result = Some(3);
                    }
                }
            }
        },
        'f' => {
            if check_slice.clone().count() >= 4 {
                let check_string = check_slice.clone().take(4).collect::<String>();
                if check_string.eq("four") {
                    result = Some(4);
                }
                if check_string.eq("five") {
                    result = Some(5);
                }
            }
        },
        's' => {
            if check_slice.clone().count() >= 3 {
                let check_string = check_slice.clone().take(3).collect::<String>();
                if check_string.eq("six") {
                    result = Some(6);
                }
                if check_slice.clone().count() >= 5 {
                    let check_string = check_slice.take(5).collect::<String>();
                    if check_string.eq("seven") {
                        result = Some(7);
                    }
                }
            }
        },
        'e' => {
            if check_slice.clone().count() >= 5 {
                let check_string = check_slice.clone().take(5).collect::<String>();
                if check_string.eq("eight") {
                    result = Some(8);
                }
            }
        },
        'n' => {
            if check_slice.clone().count() >= 4 {
                let check_string = check_slice.clone().take(4).collect::<String>();
                if check_string.eq("nine") {
                    result = Some(9);
                }
            }
        },
        _ => result = None,
    }

    result
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_is_number() {
        let char = '1';
        let result = is_number(&char);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_check_word_number() {
        let string = String::from("eighttwo2twofour9");
        let char = 'e';
        let index = 0;
        let slice = string.chars().skip(0).take(5).collect::<String>();
        println!("{:?}", slice);
        let result = check_word_number(&string, &char, index).unwrap();
        assert_eq!(result, 8);

        let string = String::from("qqqvtjxnbgseven8nvmbxsvtblqml4seven1z");
        let char = 's';
        let index = 10;
        let slice = string.chars().skip(10).take(5).collect::<String>();
        println!("{:?}", slice);
        let result = check_word_number(&string, &char, index).unwrap();
        assert_eq!(result, 7);
    }
}
