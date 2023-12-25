// We're reading scratch card numbers from a text file.
// Each "card" has 2 lists of numbers, one for the winning numbers and one for the numbers on the card.
// We need to check each card to see if it's a winner.
// If it is, we need to find the number of winning numbers on the card - each win doubles the score
// of the card.

use crate::util::read_input;

struct Card {
    winning_numbers: Vec<i32>,
    card_numbers: Vec<i32>
}

impl Card {
    /// Get the number of matches and the score of the card.
    fn get_score(&self) -> i32 {
        // Get the score by calculating the number of winning numbers on the card.
        // Each win doubles the score of the card, going 1, 2, 4...
        
        let num_matches = self.card_numbers.iter().filter(|x| self.winning_numbers.contains(x)).count() as i32;

        if num_matches != 0 {
            2_i32.pow((num_matches - 1) as u32)
        } else {
            0
        }
    }

    fn num_matches(&self) -> i32 {
        self.card_numbers.iter().filter(|x| self.winning_numbers.contains(x)).count() as i32
    }
}

pub fn run() -> std::io::Result<()> {
    // Read the file.
    let file_string = read_input("04")?;

    // Split the file into lines.
    let lines = file_string.lines().collect::<Vec<&str>>();

    // Split each line into the winning numbers and the card numbers.
    let lines = lines.iter().map(|x| {
        x.split(": ").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>()
    });

    let mut cards: Vec<Card> = Vec::new();

    for (i, line) in lines.enumerate() {
        let winning_numbers = line[0].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let card_numbers = line[1].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        // Print the numbers.
        println!("Card {} numbers: {:?} - Winning numbers: {:?}", i, card_numbers, winning_numbers);

        // Create a new Card.
        cards.push(Card {
            winning_numbers,
            card_numbers
        });
    }

    // Get the score of all cards.
    let total_score = cards.iter().map(|x| x.get_score()).sum::<i32>();

    println!("Part one total score: {}", total_score);


    // Part two: when cards are winners, they win the next x cards, where x is the number of winning numbers on the card.

    // Create a Vec to store the total number of cards in the stack.
    let mut tmp_cards = vec![1; cards.len()];

    // Populate the Vec by finding the number of matches.
    for (i, card) in cards.iter().enumerate() {
        // Get the number of matches.
        let num_matches = card.num_matches();

        // Add the number of matches to the Vec - remember that there may be multiple instances of
        // the same card.
        for j in i+1..i+1+num_matches as usize {
            tmp_cards[j] += tmp_cards[i];
        }
    }

    // Get the total number of cards.
    let total_cards = tmp_cards.iter().sum::<i32>();

    println!("Part two total cards: {}", total_cards);

    Ok(())
}
