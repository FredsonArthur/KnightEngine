#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    Pawn, Knight, Bishop, Rook, Queen, King,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Board {
    pub white_pawns: u64, pub white_knights: u64, pub white_bishops: u64,
    pub white_rooks: u64, pub white_queens: u64, pub white_king: u64,
    pub black_pawns: u64, pub black_knights: u64, pub black_bishops: u64,
    pub black_rooks: u64, pub black_queens: u64, pub black_king: u64,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white_pawns: 0xFF00, white_knights: 0x42, white_bishops: 0x24,
            white_rooks: 0x81, white_queens: 0x8, white_king: 0x10,
            black_pawns: 0xFF000000000000, black_knights: 0x4200000000000000,
            black_bishops: 0x2400000000000000, black_rooks: 0x8100000000000000,
            black_queens: 0x800000000000000, black_king: 0x1000000000000000,
        }
    }

    pub fn get_piece_at(&self, square: u64) -> Option<(PieceType, Color)> {
        if (self.white_pawns & square) != 0 { Some((PieceType::Pawn, Color::White)) }
        else if (self.white_knights & square) != 0 { Some((PieceType::Knight, Color::White)) }
        else if (self.white_bishops & square) != 0 { Some((PieceType::Bishop, Color::White)) }
        else if (self.white_rooks & square) != 0 { Some((PieceType::Rook, Color::White)) }
        else if (self.white_queens & square) != 0 { Some((PieceType::Queen, Color::White)) }
        else if (self.white_king & square) != 0 { Some((PieceType::King, Color::White)) }
        else if (self.black_pawns & square) != 0 { Some((PieceType::Pawn, Color::Black)) }
        else if (self.black_knights & square) != 0 { Some((PieceType::Knight, Color::Black)) }
        else if (self.black_bishops & square) != 0 { Some((PieceType::Bishop, Color::Black)) }
        else if (self.black_rooks & square) != 0 { Some((PieceType::Rook, Color::Black)) }
        else if (self.black_queens & square) != 0 { Some((PieceType::Queen, Color::Black)) }
        else if (self.black_king & square) != 0 { Some((PieceType::King, Color::Black)) }
        else { None }
    }

    pub fn get_all_pieces(&self) -> u64 {
        self.white_pawns | self.white_knights | self.white_bishops | 
        self.white_rooks | self.white_queens | self.white_king |
        self.black_pawns | self.black_knights | self.black_bishops | 
        self.black_rooks | self.black_queens | self.black_king
    }

    pub fn make_move(&mut self, from: u64, to: u64) {
        // 1. Obtém os dados e descarta a referência ao tabuleiro
        let piece_data = match self.get_piece_at(from) {
            Some(data) => data,
            None => return,
        };

        // 2. Remove a peça da origem
        match piece_data {
            (PieceType::Pawn, Color::White) => self.white_pawns &= !from,
            (PieceType::Knight, Color::White) => self.white_knights &= !from,
            (PieceType::Bishop, Color::White) => self.white_bishops &= !from,
            (PieceType::Rook, Color::White) => self.white_rooks &= !from,
            (PieceType::Queen, Color::White) => self.white_queens &= !from,
            (PieceType::King, Color::White) => self.white_king &= !from,
            (PieceType::Pawn, Color::Black) => self.black_pawns &= !from,
            (PieceType::Knight, Color::Black) => self.black_knights &= !from,
            (PieceType::Bishop, Color::Black) => self.black_bishops &= !from,
            (PieceType::Rook, Color::Black) => self.black_rooks &= !from,
            (PieceType::Queen, Color::Black) => self.black_queens &= !from,
            (PieceType::King, Color::Black) => self.black_king &= !from,
        }

        // 3. Limpa o destino (captura)
        self.white_pawns &= !to; self.white_knights &= !to; self.white_bishops &= !to;
        self.white_rooks &= !to; self.white_queens &= !to; self.white_king &= !to;
        self.black_pawns &= !to; self.black_knights &= !to; self.black_bishops &= !to;
        self.black_rooks &= !to; self.black_queens &= !to; self.black_king &= !to;

        // 4. Coloca a peça no destino
        match piece_data {
            (PieceType::Pawn, Color::White) => self.white_pawns |= to,
            (PieceType::Knight, Color::White) => self.white_knights |= to,
            (PieceType::Bishop, Color::White) => self.white_bishops |= to,
            (PieceType::Rook, Color::White) => self.white_rooks |= to,
            (PieceType::Queen, Color::White) => self.white_queens |= to,
            (PieceType::King, Color::White) => self.white_king |= to,
            (PieceType::Pawn, Color::Black) => self.black_pawns |= to,
            (PieceType::Knight, Color::Black) => self.black_knights |= to,
            (PieceType::Bishop, Color::Black) => self.black_bishops |= to,
            (PieceType::Rook, Color::Black) => self.black_rooks |= to,
            (PieceType::Queen, Color::Black) => self.black_queens |= to,
            (PieceType::King, Color::Black) => self.black_king |= to,
        }
    }

    pub fn print_board(&self) {
        let all = self.get_all_pieces();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = 1u64 << (rank * 8 + file);
                if (all & square) != 0 { print!(" 1 "); } else { print!(" 0 "); }
            }
            println!();
        }
    }

    pub fn print_attacks(_attacks: u64) {
        // Implementação omitida, usando _attacks para ignorar aviso de unused variable
    }
}