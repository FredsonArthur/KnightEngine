pub struct MoveGenerator;

const NOT_A_FILE: u64 = 0xFEFEFEFEFEFEFEFE;
const NOT_AB_FILE: u64 = 0xFCFCFCFCFCFCFCFC;
const NOT_H_FILE: u64 = 0x7F7F7F7F7F7F7F7F;
const NOT_GH_FILE: u64 = 0x3F3F3F3F3F3F3F3F;

impl MoveGenerator {
    pub fn get_legal_knight_moves(square: u64, friendly_pieces: u64) -> u64 {
        let attacks = Self::generate_knight_attacks(square);
        attacks & !friendly_pieces
    }

    fn generate_knight_attacks(sq: u64) -> u64 {
        (sq << 17 & NOT_A_FILE) | (sq << 15 & NOT_H_FILE) |
        (sq << 10 & NOT_AB_FILE) | (sq << 6 & NOT_GH_FILE) |
        (sq >> 17 & NOT_H_FILE) | (sq >> 15 & NOT_A_FILE) |
        (sq >> 10 & NOT_GH_FILE) | (sq >> 6 & NOT_AB_FILE)
    }
}