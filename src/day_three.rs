// All credit for this solution goes to manhunto, who posted it on the Advent of Code subreddit.
// It isn't a direct copy, check out the original here: https://github.com/manhunto/advent-of-code-rs/blob/master/src/solutions/day03.rs 
// I've added comments (which they didn't have), changed their recognize_numbers() function to
// parse_string(), and changed their part_one() and part_two() functions to run() with a println!()


use crate::util::read_input;

// Stores the number and the coordinates of the cells that contain it.
#[derive(Debug, PartialEq)]
struct Number {
    number: i32,
    positions: Vec<(i32, i32)>  // Use i32 because Symbol uses it.
}

impl Number {
    // Returns true if this number collides with the given symbol.
    fn collides_with(&self, sym: &Symbol) -> bool {
        for pos in sym.adj_coords {
            if self.positions.contains(&pos) {
                return true;
            }
        }

        false
    }

    // Returns true if this number collides with any of the given symbols.
    fn collide_with_any(&self, symbols: &Vec<Symbol>) -> bool {
        for sym in symbols {
            if self.collides_with(sym) {
                return true;
            }
        }

        false
    }
}

// Stores the coordinates of the cells that are adjacent to this symbol.
struct Symbol {
    symbol: char,
    adj_coords: [(i32, i32); 8]  // We use i32 because it's ok to store negative values.
}

impl Symbol {
    // Returns a new Symbol struct. Builds the adj_coords based on given coords.
    fn new(x: i32, y: i32, symbol: char) -> Self {
        Self {
            symbol,
            adj_coords: [
                (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                (x - 1, y),                 (x + 1, y),
                (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)
            ]
        }
    }
}

/// Parses the input string into a vector of Number and Symbol structs.
fn parse_string(input: &String) -> (Vec<Number>, Vec<Symbol>) {
    let mut tmp_numbers: Vec<char> = Vec::new();
    let mut tmp_positions: Vec<(i32, i32)> = Vec::new();

    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    // Iterate over the lines and parse them into numbers.
    for (y, line) in input.lines().enumerate() {
        // Check each character in the line.
        // If it's a number, parse it into a Number struct.
        let line = line.chars().collect::<Vec<char>>();

        for (x, c) in line.iter().enumerate() {
            if c.is_numeric() {
                // Push the number and its coordinates into the temp vectors.
                tmp_numbers.push(*c);
                tmp_positions.push((x.try_into().unwrap(), y.try_into().unwrap()));
            } else {
                // If the number vector isn't empty, parse it into a Number struct.
                if !tmp_numbers.is_empty() {
                    let number = Number {
                        number: String::from_iter(&tmp_numbers).parse::<i32>().unwrap(),
                        positions: tmp_positions.clone()
                    };

                    // Clear the temp vectors, ready for the next number.
                    tmp_numbers.clear();
                    tmp_positions.clear();

                    // Push the number into the numbers vector.
                    numbers.push(number);
                }
                
                // If the character is a symbol, parse it into a Symbol struct.
                if c != &'.' {
                    symbols.push(Symbol::new(x.try_into().unwrap(), y.try_into().unwrap(), *c));
                }
            }
        }

        // Check the number vector again, in case the line ended with a number.
        if !tmp_numbers.is_empty() {
            let number = Number {
                number: String::from_iter(&tmp_numbers).parse::<i32>().unwrap(),
                positions: tmp_positions.clone()
            };

            // Clear the temp vectors, ready for the next number.
            tmp_numbers.clear();
            tmp_positions.clear();

            // Push the number into the numbers vector.
            numbers.push(number);
        }
    }
    
    // Return the vector of numbers.
    (numbers, symbols)
}

pub fn run() -> std::io::Result<()> {
    // Open the file.
    let file_string = read_input("03")?;

    let numbers: Vec<Number>;
    let symbols: Vec<Symbol>;

    // Parse the input into the correct structs.
    (numbers, symbols) = parse_string(&file_string);

    // Part one - find the sum of all numbers that collide with symbols.
    let sum_one = numbers
        .iter()
        .filter(|n| n.collide_with_any(&symbols))
        .fold(0, |acc, n| acc + n.number);

    // Part two - find the sum of the product of all numbers where the symbol is a *, and has 2 or more
    // collisions.
    let sum_two = symbols
        .iter()
        .filter(|s| s.symbol == '*')
        .map(|s| {
            let collisions = numbers
                .iter()
                .filter(|n| n.collides_with(s))
                .map(|n| n.number);

            if collisions.clone().count() >= 2 {
                return collisions.product();
            }

            0
        })
        .sum::<i32>();
    

    println!("Part one: {}", sum_one);
    println!("Part two: {}", sum_two);

    Ok(())
}
