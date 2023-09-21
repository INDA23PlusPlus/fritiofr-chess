use crate::{errors::FromFenError, Piece};

#[derive(Debug, Copy, Clone)]
pub struct Board {
    tiles: [Option<Piece>; 64],
}

impl Board {
    /// Parses the board part of a FEN string
    ///
    /// # Arguments
    /// * `fen` - The board part of a FEN string
    ///
    /// # Examples
    /// ```
    /// // The starting position
    /// Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    /// ```
    pub fn from_fen(fen: &str) -> Result<Board, FromFenError> {
        let mut tiles: [Option<Piece>; 64] = [None; 64];

        let rows = fen.split('/').collect::<Vec<&str>>();

        if rows.len() != 8 {
            return Err(FromFenError::IncorrectAmountOfSlash);
        }

        let mut i = 0;
        for (row_index, row) in rows.iter().enumerate() {
            for c in row.chars() {
                let parsed_value = c.to_string().parse::<usize>();

                if i >= row_index * 8 + 8 {
                    return Err(FromFenError::IncorrectAmountOfTiles);
                }

                if let Ok(n) = parsed_value {
                    i += n;
                } else {
                    let piece = Piece::try_from(c).map_err(|_| FromFenError::UnknownCharacter)?;

                    tiles[i] = Some(piece);

                    i += 1;
                }
            }
        }

        if i != 64 {
            return Err(FromFenError::IncorrectAmountOfTiles);
        }

        Ok(Board { tiles })
    }

    /// Returns a piece on the board
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the tile
    /// * `y` - The y coordinate of the tile
    ///
    /// # Returns
    /// * `Option<Piece>` - The piece on the tile or None if there is no piece
    pub fn get_tile(&self, x: usize, y: usize) -> Option<Piece> {
        if x > 7 || y > 7 {
            panic!("x and y must be between 0 and 7");
        }

        let index = y * 8 + x;

        self.tiles[index]
    }

    /// Sets a tile on the board
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the tile
    /// * `y` - The y coordinate of the tile
    /// * `piece` - The piece to set the tile to
    pub fn set_tile(&mut self, x: usize, y: usize, piece: Piece) {
        if x > 7 || y > 7 {
            panic!("x and y must be between 0 and 7");
        }

        let index = y * 8 + x;

        self.tiles[index] = Some(piece);
    }

    /// Removes a tile from the board
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the tile
    /// * `y` - The y coordinate of the tile
    pub fn remove_tile(&mut self, x: usize, y: usize) {
        if x > 7 || y > 7 {
            panic!("x and y must be between 0 and 7");
        }

        let index = y * 8 + x;

        self.tiles[index] = None;
    }
}

impl Eq for Board {}
impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.tiles
            .into_iter()
            .zip(other.tiles.into_iter())
            .all(|(a, b)| a == b)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut game_string = String::new();

        for (i, tile) in self.tiles.iter().enumerate() {
            if i % 8 == 0 && i != 0 {
                game_string.push_str("\n");
            }

            if let Some(piece) = tile {
                let piece_char: char = (*piece).into();

                game_string.push(piece_char);
            } else {
                game_string.push('-');
            }
        }

        write!(f, "{}", game_string)
    }
}