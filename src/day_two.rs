// Find the id's of all games that are possible.
// There are only 12 red dice, 13 green dice, and 14 blue dice.
//
// After finding the id's, find the sum of all of them together.

// For the second part - in each game played, what is the fewest
// number of dice of each color that could have been in the bag to
// make the game valid?
// To show your knowledge of the game, get the "power" of each game,
// which is found by multiplying the minimum number of dice of each
// color together. This question does not worry about whether the game
// is valid or not.

#[derive(Debug)]
struct Game {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

pub fn run() -> std::io::Result<()> {
    let file = std::fs::read_to_string("C:\\Users\\lvas4\\Documents\\advent-rust-23\\day_two\\input.txt")?;
    let mut games: Vec<Game> = Vec::new();  // Create a Vec to store the valid games.
    
    for (id, line) in file.lines().enumerate() {
        // We split with +2 because we also want to skip the space after the colon.
        let round_string = line.chars().skip(line.find(":").unwrap() + 2).collect::<String>();
        // let rounds = line.split(":").collect::<Vec<&str>>();
        let rounds = round_string.split("; ").collect::<Vec<&str>>();
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for round in rounds {
            let round = round.split(", ").collect::<Vec<&str>>();
            // println!("{:?}", round);
            
            // The rounds do not have a specific order that the colors are in, so we need to find,
            // one by one, which color is which.
            for color in round {
                // Split the color into the color and the number.
                let color_vec = color.split(" ").collect::<Vec<&str>>();
                // println!("{:?}", color);
                let color = color_vec[1];
                let number = color_vec[0].parse::<i32>().unwrap();
                if color.contains("red") {
                    if max_red < number {
                        max_red = number;
                    }
                }
                else if color.contains("green") {
                    if max_green < number {
                        max_green = number;
                    }
                }
                else if color.contains("blue") {
                    if max_blue < number {
                        max_blue = number;
                    }
                }
            }
        }

        let game = Game {
            id: (id as i32) + 1,
            red: max_red,
            green: max_green,
            blue: max_blue,
        };

        games.push(game);
    }

    let mut sum = 0;
    let mut power_sum = 0;

    // Check for the valid games and add their id's to a sum.
    for game in games {
        if check_valid_game(&game) {
            // println!("Valid: {:?}", game);
            sum += game.id;
        }
        // Get the power of each game and add them to the power_sum
        power_sum += game.red * game.green * game.blue;
    }

    println!("Day Two:\n Sum of all valid games: {}\n Sum of all game powers: {}", sum, power_sum);

    Ok(())
}

// Verified functional.
fn check_valid_game(game: &Game) -> bool {
    if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_valid_game() {
        let valid_game = Game {
            id: 0,
            red: 12,
            green: 13,
            blue: 14,
        };

        let invalid_game = Game {
            id: 0,
            red: 13,
            green: 13,
            blue: 14,
        };

        assert_eq!(check_valid_game(&valid_game), true);
        assert_eq!(check_valid_game(&invalid_game), false);
    }
}
