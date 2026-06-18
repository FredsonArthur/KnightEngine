/// Módulo de testes unitários para validar a lógica de movimento e captura de cada peça.
/// Este módulo é compilado apenas durante o comando `cargo test`.
#[cfg(test)]
mod tests {
    use crate::moves::MoveGenerator;
    use crate::board::{Board, Color};

    /// Testa se o cavalo respeita a presença de peças aliadas,
    /// não permitindo movimentos para casas já ocupadas pelo próprio time.
    #[test]
    fn test_knight_friendly_blockage() {
        let d4 = 1u64 << 27;
        let f5 = 1u64 << 37;
        let aliados = f5; 
        
        let moves = MoveGenerator::get_legal_knight_moves(d4, aliados);
        
        assert_eq!(moves & f5, 0, "O cavalo não deve poder pular em uma casa ocupada por aliado");
    }

    /// Valida se o cavalo respeita as bordas do tabuleiro, não realizando movimentos ilegais.
    #[test]
    fn test_knight_bounds() {
        let a1 = 1u64 << 0;
        let moves = MoveGenerator::get_legal_knight_moves(a1, 0);
        
        assert!((moves & (1u64 << 10)) != 0); // b3
        assert!((moves & (1u64 << 17)) != 0); // c2
        assert_eq!(moves.count_ones(), 2, "Cavalo em a1 deve ter apenas 2 movimentos legais");
    }

    /// Testa a lógica de deslizamento da Torre, verificando se ela é bloqueada
    /// por peças inimigas ou aliadas no caminho.
    #[test]
    fn test_rook_blockage() {
        let d4 = 1u64 << 27;
        let blocker = 1u64 << 43; // d6
        let occupancy = blocker;
        
        let moves = MoveGenerator::generate_rook_attacks(d4, occupancy);
        
        assert!((moves & (1u64 << 35)) != 0, "Torre deve atacar d5");
        assert!((moves & blocker) != 0, "Torre deve atacar a casa do bloqueador (captura)");
        assert!((moves & (1u64 << 51)) == 0, "Torre NÃO deve pular a peça em d6");
    }

    /// Valida a movimentação diagonal do Bispo e seu comportamento de bloqueio.
    #[test]
    fn test_bishop_blockage() {
        let d4 = 1u64 << 27;
        let blocker = 1u64 << 45; // f6
        let occupancy = blocker;
        
        let moves = MoveGenerator::generate_bishop_attacks(d4, occupancy);
        
        assert!((moves & (1u64 << 36)) != 0, "Bispo deve atacar e5");
        assert!((moves & blocker) != 0, "Bispo deve atacar a casa do bloqueador");
        assert!((moves & (1u64 << 54)) == 0, "Bispo NÃO deve pular a peça em f6");
    }

    /// Testa a Rainha, que combina as lógicas de Torre e Bispo, 
    /// garantindo que ambas as direções (ortogonais e diagonais) sejam bloqueadas corretamente.
    #[test]
    fn test_queen_blockage() {
        let d4 = 1u64 << 27;
        let blocker_n = 1u64 << 43;
        let blocker_ne = 1u64 << 45;
        let occupancy = blocker_n | blocker_ne;
        
        let moves = MoveGenerator::generate_queen_attacks(d4, occupancy);
        
        assert!((moves & blocker_n) != 0, "Rainha deve atacar o bloqueador ao Norte");
        assert!((moves & blocker_ne) != 0, "Rainha deve atacar o bloqueador ao Nordeste");
        
        assert!((moves & (1u64 << 51)) == 0, "Rainha não deve pular bloqueador no Norte");
        assert!((moves & (1u64 << 54)) == 0, "Rainha não deve pular bloqueador no Nordeste");
    }

    /// Testa se o Rei não consegue se mover para fora do tabuleiro ao estar em um canto.
    #[test]
    fn test_king_bounds() {
        let a1 = 1u64 << 0;
        let moves = MoveGenerator::generate_king_attacks(a1);
        
        assert!((moves & (1u64 << 1)) != 0); // b1
        assert!((moves & (1u64 << 8)) != 0); // a2
        assert!((moves & (1u64 << 9)) != 0); // b2
        
        assert_eq!(moves.count_ones(), 3);
    }

    /// Testa o avanço simples e duplo do peão branco, verificando a condição de casa vazia.
    #[test]
    fn test_pawn_white_push() {
        let e2 = 1u64 << 12; 
        let occupancy = 0u64;
        let moves = MoveGenerator::generate_pawn_pushes(e2, Color::White, occupancy);
        
        assert!((moves & (1u64 << 20)) != 0, "Peão e2 deve poder avançar para e3");
        assert!((moves & (1u64 << 28)) != 0, "Peão e2 deve poder avançar para e4");
    }
}