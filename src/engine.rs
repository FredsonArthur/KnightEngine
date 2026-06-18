use crate::board::{Board, PieceType, Color};
use crate::moves::MoveGenerator;

pub struct Engine;

impl Engine {
    pub fn is_square_attacked(square: u64, attacker_color: Color, board: &Board) -> bool {
        let enemy_occupancy = if attacker_color == Color::White {
            board.white_pawns | board.white_knights | board.white_bishops | 
            board.white_rooks | board.white_queens | board.white_king
        } else {
            board.black_pawns | board.black_knights | board.black_bishops | 
            board.black_rooks | board.black_queens | board.black_king
        };

        if (MoveGenerator::generate_knight_attacks(square) & enemy_occupancy) != 0 { return true; }
        if (MoveGenerator::generate_rook_attacks(square, board.get_all_pieces()) & enemy_occupancy) != 0 { return true; }
        if (MoveGenerator::generate_bishop_attacks(square, board.get_all_pieces()) & enemy_occupancy) != 0 { return true; }
        if (MoveGenerator::generate_king_attacks(square) & enemy_occupancy) != 0 { return true; }
        false
    }

    pub fn is_in_check(color: Color, board: &Board) -> bool {
        let king_square = if color == Color::White { board.white_king } else { board.black_king };
        let attacker = if color == Color::White { Color::Black } else { Color::White };
        Self::is_square_attacked(king_square, attacker, board)
    }

    pub fn get_legal_moves(square: u64, board: &Board) -> u64 {
        let piece_info = match board.get_piece_at(square) {
            Some(info) => info,
            None => return 0,
        };

        let (piece_type, color) = piece_info;
        let occupancy = board.get_all_pieces();
        
        let raw_moves = match piece_type {
            PieceType::Pawn => MoveGenerator::generate_pawn_pushes(square, color, occupancy) | 
                               MoveGenerator::generate_pawn_captures(square, color),
            PieceType::Knight => MoveGenerator::generate_knight_attacks(square),
            PieceType::Bishop => MoveGenerator::generate_bishop_attacks(square, occupancy),
            PieceType::Rook => MoveGenerator::generate_rook_attacks(square, occupancy),
            PieceType::Queen => MoveGenerator::generate_queen_attacks(square, occupancy),
            PieceType::King => MoveGenerator::generate_king_attacks(square),
        };

        let friendly_pieces = if color == Color::White {
            board.white_pawns | board.white_knights | board.white_bishops | 
            board.white_rooks | board.white_queens | board.white_king
        } else {
            board.black_pawns | board.black_knights | board.black_bishops | 
            board.black_rooks | board.black_queens | board.black_king
        };

        let possible_moves = raw_moves & !friendly_pieces;
        let mut legal_moves = 0u64;

        for i in 0..64 {
            let target = 1u64 << i;
            if (possible_moves & target) != 0 {
                let mut board_simulado = *board; // Usa a cópia do Board (Copy trait)
                board_simulado.make_move(square, target);
                
                if !Self::is_in_check(color, &board_simulado) {
                    legal_moves |= target;
                }
            }
        }
        legal_moves
    }

    pub fn get_friendly_pieces(board: &Board, color: Color) -> u64 {
        if color == Color::White {
            board.white_pawns | board.white_knights | board.white_bishops | 
            board.white_rooks | board.white_queens | board.white_king
        } else {
            board.black_pawns | board.black_knights | board.black_bishops | 
            board.black_rooks | board.black_queens | board.black_king
        }
    }
}