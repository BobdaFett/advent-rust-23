// This is a matrix counting problem. Our objective is to find all of the numbers in
// the matrix that are adjacent to a symbol. A number is considered adjacent if it is
// directly above, below, left, right, or diagonal to a symbol. We can solve this by
// creating a 2d array of Option<(char, bool)> where the bool represents whether or
// not that cell has been checked.


pub fn run() -> std::io::Result<()> {
    // Read the file into a string.
    let input = std::fs::read_to_string("C:\\Users\\lvas4\\Documents\\advent-rust-23\\day_three\\input.txt")?;

    // Create the matrix.
    // let mut matrix: Vec<Vec<Option<(char, bool)>>> = vec![vec![None; 1]; 1];
    let mut matrix: Vec<Vec<Option<(char, bool)>>> = Vec::new();

    // Iterate through each line of the input.
    for (row, line) in input.lines().enumerate() {
        // Push a new row onto the matrix.
        matrix.push(vec![None; line.len()]);

        // Split the line into a Vec of chars.
        let chars = line.chars().collect::<Vec<char>>();

        // Build the matrix.
        for (col, char) in chars.into_iter().enumerate() {
            // If the char is a symbol or number, set the cell to Some((char, false)).
            if char != '.' {
                matrix[row][col] = Some((char, false));
            }
        }
    }

    // Matrix should be built.
    // print_matrix(&matrix);
    // print_tf_matrix(&matrix);
    
    // TODO Iterate through the matrix and check every cell that is Some((char, false)).
    for (row, row_vec) in matrix.iter_mut().enumerate() {
        for (col, cell) in row_vec.iter_mut().enumerate() {
            // If the cell is Some((char, false)), check it.
            if let Some((char, checked)) = cell {
                // Check the cell to see if it is a symbol.
                if !char.is_ascii_digit() {
                    // Check around the cell for numbers. This is 8 consecutive checks.

                    // Check row above.

                    // Check current row.
                    
                    // Check row below.
                }
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn print_matrix(matrix: &Vec<Vec<Option<(char, bool)>>>) {
    for row in matrix {
        for cell in row {
            if let Some((char, _)) = cell {
                print!("{}", char);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_tf_matrix(matrix: &Vec<Vec<Option<(char, bool)>>>) {
    for row in matrix {
        for cell in row {
            if let Some((_, checked)) = cell {
                if *checked {
                    print!("T");
                } else {
                    print!("F");
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}
