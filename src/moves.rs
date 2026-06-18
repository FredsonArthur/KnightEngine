/// `MoveGenerator` é uma estrutura utilitária que contém toda a lógica 
/// para gerar ataques e movimentos legais para cada peça do xadrez.
pub struct MoveGenerator;

// Máscaras de arquivos (colunas) para evitar o "efeito wrap-around" 
// (quando um bit deslocado atravessa o tabuleiro indevidamente).
const NOT_A_FILE: u64 = 0xFEFEFEFEFEFEFEFE;
const NOT_AB_FILE: u64 = 0xFCFCFCFCFCFCFCFC;
const NOT_H_FILE: u64 = 0x7F7F7F7F7F7F7F7F;
const NOT_GH_FILE: u64 = 0x3F3F3F3F3F3F3F3F;

impl MoveGenerator {
    /// Filtra os ataques do cavalo, garantindo que ele não capture peças aliadas.
    pub fn get_legal_knight_moves(square: u64, friendly_pieces: u64) -> u64 {
        let attacks = Self::generate_knight_attacks(square);
        attacks & !friendly_pieces
    }

    /// Calcula os movimentos básicos do cavalo usando saltos de bits pré-definidos.
    fn generate_knight_attacks(sq: u64) -> u64 {
        (sq << 17 & NOT_A_FILE) | (sq << 15 & NOT_H_FILE) |
        (sq << 10 & NOT_AB_FILE) | (sq << 6 & NOT_GH_FILE) |
        (sq >> 17 & NOT_H_FILE) | (sq >> 15 & NOT_A_FILE) |
        (sq >> 10 & NOT_GH_FILE) | (sq >> 6 & NOT_AB_FILE)
    }

    /// Gera ataques de peças deslizantes ortogonais (Torre).
    /// Percorre as 4 direções cardinais até encontrar uma borda ou peça.
    pub fn generate_rook_attacks(square: u64, occupancy: u64) -> u64 {
        let mut attacks = 0u64;
        let directions: [i32; 4] = [8, -8, 1, -1];

        for dir in directions {
            let mut current = square;
            loop {
                // Impede que a peça salte da coluna H para A ou vice-versa
                if dir == 1 && (current & 0x8080808080808080) != 0 { break; } 
                if dir == -1 && (current & 0x0101010101010101) != 0 { break; }

                current = if dir > 0 { 
                    current << dir as u32 
                } else { 
                    current >> dir.abs() as u32 
                };
                
                if current == 0 { break; }
                attacks |= current;

                // Se houver peça no caminho, a torre para após incluir a casa (captura)
                if (current & occupancy) != 0 { break; }
            }
        }
        attacks
    }

    /// Gera ataques de peças deslizantes diagonais (Bispo).
    /// Similar à torre, mas com deslocamentos diagonais.
    pub fn generate_bishop_attacks(square: u64, occupancy: u64) -> u64 {
        let mut attacks = 0u64;
        let directions: [i32; 4] = [9, -7, -9, 7];

        for dir in directions {
            let mut current = square;
            loop {
                // Impede travessias diagonais ilegais nas bordas A e H
                if dir == 9 || dir == -7 { 
                    if (current & 0x8080808080808080) != 0 { break; } 
                }
                if dir == -9 || dir == 7 { 
                    if (current & 0x0101010101010101) != 0 { break; }
                }

                current = if dir > 0 { 
                    current << dir as u32 
                } else { 
                    current >> dir.abs() as u32 
                };
                
                if current == 0 { break; }
                attacks |= current;

                if (current & occupancy) != 0 { break; }
            }
        }
        attacks
    }

    /// A Rainha combina os ataques da Torre e do Bispo (operação OR bitwise).
    pub fn generate_queen_attacks(square: u64, occupancy: u64) -> u64 {
        Self::generate_rook_attacks(square, occupancy) | 
        Self::generate_bishop_attacks(square, occupancy)
    }

    /// Gera ataques para o Rei, cobrindo as 8 casas adjacentes.
    pub fn generate_king_attacks(square: u64) -> u64 {
        let mut attacks = 0u64;
        let not_a_file = 0xFEFEFEFEFEFEFEFE;
        let not_h_file = 0x7F7F7F7F7F7F7F7F;

        // Deslocamentos simples aplicados com máscaras de borda
        attacks |= (square << 8) | (square >> 8);
        attacks |= (square << 1 & not_a_file) | (square >> 1 & not_h_file);
        attacks |= (square << 9 & not_a_file) | (square << 7 & not_h_file);
        attacks |= (square >> 9 & not_h_file) | (square >> 7 & not_a_file);

        attacks
    }

    /// Gera as casas atacadas diagonalmente por um peão (usadas para captura).
    pub fn generate_pawn_captures(square: u64, color: &str) -> u64 {
        if color == "white" {
            ((square << 7) & 0x7F7F7F7F7F7F7F7F) | ((square << 9) & 0xFEFEFEFEFEFEFEFE)
        } else {
            ((square >> 7) & 0xFEFEFEFEFEFEFEFE) | ((square >> 9) & 0x7F7F7F7F7F7F7F7F)
        }
    }

    /// Calcula movimentos de avanço do peão, considerando bloqueios e avanço duplo.
    pub fn generate_pawn_pushes(square: u64, color: &str, occupancy: u64) -> u64 {
        let mut pushes = 0u64;
        let forward: i32 = if color == "white" { 8 } else { -8 };

        // Calcula casa imediatamente à frente
        let single = if forward > 0 { 
            square << forward as u32 
        } else { 
            square >> forward.abs() as u32 
        };
        
        // Só pode avançar se a casa estiver desocupada
        if (single & occupancy) == 0 {
            pushes |= single;
            
            // Avanço duplo: verificação de rank inicial e casas livres
            let double = if forward > 0 { single << 8 } else { single >> 8 };
            let rank_start = if color == "white" { 0xFF00 } else { 0xFF000000000000 };
            
            if (square & rank_start) != 0 && (double & occupancy) == 0 {
                pushes |= double;
            }
        }
        pushes
    }
}