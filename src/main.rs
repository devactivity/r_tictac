#[derive(Debug)]
struct Board {
    cells: [char; 9],
}

impl Board {
    fn new() -> Self {
        Board { cells: [' '; 9] }
    }

    fn display(&self) {
        println!(
            "\n {} | {} | {} ",
            self.cells[0], self.cells[1], self.cells[2]
        );
        println!("---|---|---");
        println!(
            " {} | {} | {} ",
            self.cells[3], self.cells[4], self.cells[5]
        );
        println!("---|---|---");
        println!(
            " {} | {} | {} \n",
            self.cells[6], self.cells[7], self.cells[8]
        );
    }
}

#[derive(Debug, Clone)]
enum Player {
    X,
    O,
}

use std::io;

fn get_move() -> usize {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Parse the input and check if it's a valid number
        match input.trim().parse::<usize>() {
            Ok(num) if num < 9 => return num,
            _ => println!("Please enter a valid number between 0 and 8."),
        }
    }
}

impl Board {
    fn make_move(&mut self, player: &Player, position: usize) -> bool {
        if position < 9 && self.cells[position] == ' ' {
            self.cells[position] = match player {
                Player::X => 'X',
                Player::O => 'O',
            };
            true
        } else {
            println!("Invalid move. Position already taken or out of bounds.");
            false
        }
    }
}

impl Board {
    fn check_winner(&self) -> Option<Player> {
        let win_conditions = [
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8), // Rows
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8), // Columns
            (0, 4, 8),
            (2, 4, 6), // Diagonals
        ];

        for &(a, b, c) in win_conditions.iter() {
            if self.cells[a] != ' '
                && self.cells[a] == self.cells[b]
                && self.cells[b] == self.cells[c]
            {
                return Some(match self.cells[a] {
                    'X' => Player::X,
                    'O' => Player::O,
                    _ => unreachable!(),
                });
            }
        }
        None
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = Player::X;

    loop {
        board.display();
        println!("Player {:?}, enter your move (0-8):", current_player);
        let position = get_move();

        if board.make_move(&current_player, position) {
            if let Some(winner) = board.check_winner() {
                board.display();
                println!("Player {:?} wins!", winner);
                break;
            }
            current_player = match current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
        }
    }
}
