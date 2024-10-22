const FORMAT_LABEL: &str = "(Format: columnrow, Example: D2)";

#[derive(Clone, PartialEq)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

#[derive(Clone)]
struct Piece {
    team_white: bool,
    piece_type: PieceType,
    position: (u8, u8),
}

impl Piece {
    fn move_piece(&self, where_to_move: (u8, u8), pieces: Vec<Piece>) -> Option<Self> {
        //If any move conditions are met return piece with new position
        if self
            .move_conditions(where_to_move, pieces)
            .iter()
            .filter(|&&x| x == true)
            .count()
            != 0
        {
            Some(Piece {
                position: where_to_move,
                ..self.clone()
            })
        } else {
            None
        }
    }

    fn move_conditions(&self, where_to_move: (u8, u8), pieces: Vec<Piece>) -> Vec<bool> {
        let taken_positions = pieces
            .iter()
            .map(|piece| (piece.position.0, piece.position.1, piece.team_white))
            .collect::<Vec<(u8, u8, bool)>>();

        match self.piece_type {
            PieceType::Pawn => vec![
                //Move forward
                where_to_move
                    == (
                        self.position.0,
                        (self.position.1 as i8 + if self.team_white { -1 } else { 1 }) as u8,
                    )
                    && !taken_positions
                        .iter()
                        .map(|x| (x.0, x.1))
                        .collect::<Vec<(u8, u8)>>()
                        .contains(&(where_to_move.0, where_to_move.1)),
                //Strike diagonally
                taken_positions.contains(&(where_to_move.0, where_to_move.1, !self.team_white))
                    && (where_to_move.0 as i8 - self.position.0 as i8).abs() == 1
                    && where_to_move.1
                        == (self.position.1 as i8 + if self.team_white { -1 } else { 1 }) as u8,
                //Move forward two steps if unmoved
                self.position.1 == if self.team_white { 6 } else { 1 }
                    && where_to_move
                        == (
                            self.position.0,
                            (self.position.1 as i8 + if self.team_white { -2 } else { 2 }) as u8,
                        )
                    && !taken_positions
                        .iter()
                        .map(|x| (x.0, x.1))
                        .collect::<Vec<(u8, u8)>>()
                        .contains(&(where_to_move.0, where_to_move.1))
                    //Prevent from moving over pieces
                    && !taken_positions
                        .iter()
                        .map(|x| (x.0, x.1))
                        .collect::<Vec<(u8, u8)>>()
                        .contains(&(where_to_move.0, (where_to_move.1 as i8 + if self.team_white {1} else {-1}) as u8 )),
            ],
            PieceType::Rook => vec![
                //Move vertically
                where_to_move.0 == self.position.0
                    && where_to_move.1 as i8 - self.position.1 as i8 != 0
                    && 
                    //Ensure range begins with the smaller position
                    if where_to_move.1 > self.position.1 {
                        //Allow striking while preventing striking teammate
                        if taken_positions.contains(&(
                            where_to_move.0,
                            where_to_move.1,
                            !self.team_white,
                        )) {
                            self.position.1..where_to_move.1
                        } else {
                            self.position.1..where_to_move.1 + 1
                        }
                    } else {
                        //Allow striking while preventing striking teammate
                        if taken_positions.contains(&(
                            where_to_move.0,
                            where_to_move.1,
                            !self.team_white,
                        )) {
                            where_to_move.1 + 1..self.position.1
                        } else {
                            where_to_move.1..self.position.1
                        }
                    }
                    //Ensure no pieces in the path
                    .filter(|i| {
                        taken_positions
                            .iter()
                            .filter(|x| x.0 == self.position.0)
                            .map(|j| j.1)
                            .collect::<Vec<u8>>()
                            .contains(&i)
                    })
                    .count()
                        == 0,
                //Move horizontally
                where_to_move.1 == self.position.1
                    && where_to_move.0 as i8 - self.position.0 as i8 != 0
                    && 
                    //Ensure range begins with the lower number
                    if where_to_move.0 > self.position.0 {
                        //Allow striking while preventing striking teammate
                        if taken_positions.contains(&(
                            where_to_move.0,
                            where_to_move.1,
                            !self.team_white,
                        )) {
                            self.position.0..where_to_move.0
                        } else {
                            self.position.0..where_to_move.0 + 1
                        }
                    } else {
                        //Allow striking while preventing striking teammate
                        if taken_positions.contains(&(
                            where_to_move.0,
                            where_to_move.1,
                            !self.team_white,
                        )) {
                            where_to_move.0 + 1..self.position.0
                        } else {
                            where_to_move.0..self.position.0
                        }
                    }
                    //Ensure no pieces in path
                    .filter(|i| {
                        taken_positions
                            .iter()
                            .filter(|x| x.1 == self.position.1)
                            .map(|j| j.0)
                            .collect::<Vec<u8>>()
                            .contains(&i)
                    })
                    .count()
                        == 0,
            ],
            PieceType::Knight => vec![
                //L shape movement; horizontal 2 vertical 1
                (where_to_move.0 as i8 - self.position.0 as i8).abs() == 2
                    && (where_to_move.1 as i8 - self.position.1 as i8).abs() == 1
                    //Ensure not striking teammate
                    && !taken_positions.contains(&(
                        where_to_move.0,
                        where_to_move.1,
                        self.team_white,
                    )),
                //L shape movement; vertical 2 horizontal 1
                (where_to_move.1 as i8 - self.position.1 as i8).abs() == 2
                    && (where_to_move.0 as i8 - self.position.0 as i8).abs() == 1
                    //Ensure not striking teammate
                    && !taken_positions.contains(&(
                        where_to_move.0,
                        where_to_move.1,
                        self.team_white,
                    )),
            ],
            PieceType::Bishop => vec![
                //Ensure moving
                where_to_move.1 as i8 - self.position.1 as i8 != 0
                    //Ensure diagonal
                    && (where_to_move.0 as i8 - self.position.0 as i8).abs()
                        == (where_to_move.1 as i8 - self.position.1 as i8).abs()
                    //Ensure range begins with lower number
                    && if where_to_move.0 > self.position.0 {
                        //Allow striking while preventing from striking teammate
                        if taken_positions.contains(&(
                            where_to_move.0,
                            where_to_move.1,
                            !self.team_white,
                        )) {
                            self.position.0..where_to_move.0
                        } else {
                            self.position.0..where_to_move.0 + 1
                        }
                    } else {
                        //Allow striking while preventing from striking teammate
                        if taken_positions.contains(&(
                            where_to_move.0,
                            where_to_move.1,
                            !self.team_white,
                        )) {
                            where_to_move.0 + 1..self.position.0
                        } else {
                            where_to_move.0..self.position.0
                        }
                    }
                    //Ensure no pieces in the path; this part checks for same x pos
                    .filter(|i| {
                        taken_positions
                            .iter()
                            .filter(|j| {
                                (self.position.1 as i8 - j.1 as i8).abs()
                                    == (self.position.0 as i8 - j.0 as i8).abs()
                                    && if where_to_move.0 > self.position.0 {
                                        j.0 > self.position.0 && j.0 <= where_to_move.0
                                    } else {
                                        j.0 < self.position.0 && j.0 >= where_to_move.0
                                    }
                                    && if where_to_move.1 > self.position.1 {
                                        j.1 > self.position.1 && j.1 <= where_to_move.1
                                    } else {
                                        j.1 < self.position.1 && j.1 >= where_to_move.1
                                    }
                            })
                            .map(|x| x.0)
                            .collect::<Vec<u8>>()
                            .contains(&i)
                    })
                    .count()
                        == 0
                    //Ensure range begins with lower number
                    && if where_to_move.1 > self.position.1 {
                        //Allow striking while preventing striking teammate
                        if taken_positions.contains(&(
                            where_to_move.0,
                            where_to_move.1,
                            !self.team_white,
                        )) {
                            self.position.1..where_to_move.1
                        } else {
                            self.position.1..where_to_move.1 + 1
                        }
                    } else {
                        //Allow striking while preventing striking teammate
                        if taken_positions.contains(&(
                            where_to_move.0,
                            where_to_move.1,
                            !self.team_white,
                        )) {
                            where_to_move.1 + 1..self.position.1
                        } else {
                            where_to_move.1..self.position.1
                        }
                    }
                    //Ensure no pieces in path; this part checks for same y pos
                    .filter(|i| {
                        taken_positions
                            .iter()
                            .filter(|j| {
                                (self.position.1 as i8 - j.1 as i8).abs()
                                    == (self.position.0 as i8 - j.0 as i8).abs()
                                    && if where_to_move.0 > self.position.0 {
                                        j.0 > self.position.0 && j.0 <= where_to_move.0
                                    } else {
                                        j.0 < self.position.0 && j.0 >= where_to_move.0
                                    }
                                    && if where_to_move.1 > self.position.1 {
                                        j.1 > self.position.1 && j.1 <= where_to_move.1
                                    } else {
                                        j.1 < self.position.1 && j.1 >= where_to_move.1
                                    }
                            })
                            .map(|x| x.1)
                            .collect::<Vec<u8>>()
                            .contains(&i)
                    })
                    .count()
                        == 0,
            ],
            PieceType::Queen => [
                //Combine rook and bishop conditions 
                //Will have to remove castling once implemented.
                Piece {
                    team_white: self.team_white,
                    position: self.position,
                    piece_type: PieceType::Rook,
                }
                .move_conditions(where_to_move, pieces.clone()),
                Piece {
                    team_white: self.team_white,
                    position: self.position,
                    piece_type: PieceType::Bishop,
                }
                .move_conditions(where_to_move, pieces),
            ]
            .concat(),
            PieceType::King => vec![
                //Ensure king can't move into danger
                pieces
                    .iter()
                    .filter(|piece| piece.team_white != self.team_white)
                    .find(|piece| {
                        if piece.piece_type != PieceType::Pawn {
                            piece.move_piece(where_to_move, pieces.clone()).is_some()
                        } else {
                            //Check pawn striking separately since pawns strike differently than their regular movement
                            
                            /*
                            Combine a vec with king's potential new position and the original vec
                            without the king (I'm aware this is probably the worst imaginible way to do this)
                            */
                            piece.move_conditions(where_to_move, vec![
                                vec![
                                Piece {
                                    position: where_to_move,
                                    ..self.clone()}
                                ],
                                pieces
                                    .iter()
                                    .filter(|x| x.position != self.position)
                                        .map(|x| x.to_owned())
                                        .collect::<Vec<_>>()
                            ].concat())[1]
                        }
                    })
                    .is_none()
                    //Ensure not striking teammate
                    && !taken_positions.contains(&(
                        where_to_move.0,
                        where_to_move.1,
                        self.team_white,
                    ))
                    //Move 1 space in any direction
                    && where_to_move != self.position
                    && (where_to_move.0 as i8 - self.position.0 as i8).abs() <= 1
                    && (where_to_move.1 as i8 - self.position.1 as i8).abs() <= 1,
            ],
        }
    //TODO: castling, pins, promotions, en passant

    }
}

fn main() {
    let mut turn = 0usize;

    let mut current_pieces = default_pieces();
    let mut fallen_pieces = vec![];

    loop {
        println!("{}", render_board(current_pieces.clone()));
        
        println!("{}", if turn % 2 == 0 {
            "White's turn"
        } else {
            "Black's turn"
        });

        fn take_position_input(current_pieces: Vec<Piece>, should_contain_piece: bool) -> (u8, u8) {
            loop {
                let mut piece_to_move = String::new();
                std::io::stdin().read_line(&mut piece_to_move).unwrap();
                piece_to_move = piece_to_move.to_uppercase();

                let proper_format_regex = regex::Regex::new("^[A-H][1-8]$").unwrap();
                if proper_format_regex.is_match(piece_to_move.trim()) {
                    if current_pieces
                        .iter()
                        .map(|x| x.position)
                        .collect::<Vec<(u8, u8)>>()
                        .contains(&parse_location(piece_to_move.trim()))
                        || !should_contain_piece
                    {
                        break parse_location(piece_to_move.trim());
                    } else {
                        println!("No piece in that location. Please try again... {FORMAT_LABEL}");
                    }
                } else {
                    println!("Incorrect format. Please try again... {FORMAT_LABEL}");
                }
            }
        }

        println!("Choose piece to move... {FORMAT_LABEL}");
        let mut piece_to_move = take_position_input(current_pieces.clone(), true);

        let old_piece = current_pieces.remove(loop {
            match current_pieces
                .iter()
                .position(|x| x.position == piece_to_move)
            {
                Some(v) =>
                    if current_pieces[v].team_white == (turn % 2 == 0) {
                        break v
                    } else {
                        println!("Not your turn! Choose piece to move again... {FORMAT_LABEL}");
                        piece_to_move = take_position_input(current_pieces.clone(), true);
                        continue;
                    }
                None => {
                    println!("No piece in that position. Choose again... {FORMAT_LABEL}");
                    piece_to_move = take_position_input(current_pieces.clone(), true);
                    continue;
                }
            }
        });

        println!("Choose where to move... {FORMAT_LABEL}");
        let mut where_to_move = take_position_input(current_pieces.clone(), false);

        let new_piece = loop {
            match old_piece.move_piece(where_to_move, current_pieces.clone()) {
                Some(v) => break v,
                None => {
                    println!(
                        "Cannot move piece there. Choose where to move again... {FORMAT_LABEL}"
                    );
                    where_to_move = take_position_input(current_pieces.clone(), false);
                    continue;
                }
            }
        };

        if let Some(v) = current_pieces
            .iter()
            .position(|x| x.position == where_to_move)
        {
            fallen_pieces.push(current_pieces.remove(v));
        }

        current_pieces.push(new_piece);
        
        turn += 1;
    }
}

fn parse_location(location: &str) -> (u8, u8) {
    let chars = location.chars().collect::<Vec<char>>();
    
    let column = "ABCDEFGH".chars().position(|x| x == chars[0]).unwrap();
    let row = chars[1].to_string().parse::<u8>().unwrap() - 1;

    (column as u8, row)
}

fn default_pieces() -> Vec<Piece> {
    let mut pieces: Vec<Piece> = vec![];
    for i in 0..16 {
        pieces.push(Piece {
            team_white: i < 8, //first half are white second half are black
            piece_type: PieceType::Pawn,
            position: if i < 8 { (i, 6) } else { (i - 8, 1) }, // change position depending on team
        })
    }

    for i in 0..4 {
        pieces.push(Piece {
            team_white: i < 2, //first half are white second half are black
            piece_type: PieceType::Rook,
            position: match i {
                0 => (0, 7),
                1 => (7, 7),
                2 => (0, 0),
                3 => (7, 0),
                _ => (8, 8),
            },
        })
    }

    for i in 0..4 {
        pieces.push(Piece {
            team_white: i < 2, //first half are white second half are black
            piece_type: PieceType::Knight,
            position: match i {
                0 => (1, 7),
                1 => (6, 7),
                2 => (1, 0),
                3 => (6, 0),
                _ => (8, 8),
            },
        })
    }

    for i in 0..4 {
        pieces.push(Piece {
            team_white: i < 2, //first half are white second half are black
            piece_type: PieceType::Bishop,
            position: match i {
                0 => (2, 7),
                1 => (5, 7),
                2 => (2, 0),
                3 => (5, 0),
                _ => (8, 8),
            },
        })
    }

    for i in 0..2 {
        pieces.push(Piece {
            team_white: i < 1, //first half are white second half are black
            piece_type: PieceType::Queen,
            position: match i {
                0 => (3, 7),
                1 => (3, 0),
                _ => (8, 8),
            },
        })
    }

    for i in 0..2 {
        pieces.push(Piece {
            team_white: i < 1, //first half are white second half are black
            piece_type: PieceType::King,
            position: match i {
                0 => (4, 7),
                1 => (4, 0),
                _ => (8, 8),
            },
        })
    }

    pieces
}

fn render_board(pieces: Vec<Piece>) -> String {
    let row_label = "   A B C D E F G H\n".to_string();

    let mut board = "󰝤 󰝤 󰝤 󰝤 \n 󰝤 󰝤 󰝤 󰝤\n".repeat(4);

    for piece in pieces {
        let row = board.lines().collect::<Vec<&str>>()[piece.position.1 as usize];
        let mut row_chars = row.chars().collect::<Vec<char>>();
        row_chars[piece.position.0 as usize] = match piece.piece_type {
            PieceType::Pawn => {
                if piece.team_white {
                    ''
                } else {
                    ''
                }
            }

            PieceType::Rook => {
                if piece.team_white {
                    '󰡛'
                } else {
                    ''
                }
            }

            PieceType::Knight => {
                if piece.team_white {
                    ''
                } else {
                    ''
                }
            }

            PieceType::Bishop => {
                if piece.team_white {
                    '󰡜'
                } else {
                    ''
                }
            }

            PieceType::Queen => {
                if piece.team_white {
                    '󰡚'
                } else {
                    ''
                }
            }

            PieceType::King => {
                if piece.team_white {
                    '󰡗'
                } else {
                    ''
                }
            }
        };

        let row = &row_chars.iter().collect::<String>();

        board = board
            .lines()
            .enumerate()
            .map(|(i, v)| {
                if i == piece.position.1 as usize {
                    row.to_string() + "\n"
                } else {
                    v.to_string() + "\n"
                }
            })
            .collect::<String>();
    }

    row_label
        + &board
            .lines()
            .enumerate()
            //number the rows and add spacing
            .map(|(i, v)| {
                format!("{}{}", i + 1, v)
                    .split("")
                    .collect::<Vec<&str>>()
                    .join(" ")
                    + "\n"
            })
            .collect::<String>()
}
