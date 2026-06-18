#[cfg(test)]
mod tests {
    use crate::moves::MoveGenerator;

    #[test]
    fn test_knight_friendly_blockage() {
        // Cavalo em d4 (índice 27)
        let d4 = 1u64 << 27;
        
        // Colocamos um aliado na casa f5 (índice 37)
        // O cavalo deveria poder pular em f5, mas o aliado o impede
        let f5 = 1u64 << 37;
        let aliados = f5; 
        
        let moves = MoveGenerator::get_legal_knight_moves(d4, aliados);
        
        // Verifica se f5 NÃO está nos movimentos permitidos
        assert_eq!(moves & f5, 0, "O cavalo não deve poder pular em uma casa ocupada por aliado");
    }

    #[test]
    fn test_knight_bounds() {
        // Cavalo em a1 (índice 0)
        let a1 = 1u64 << 0;
        let moves = MoveGenerator::get_legal_knight_moves(a1, 0);
        
        // a1 ataca b3 (10) e c2 (17)
        assert!((moves & (1u64 << 10)) != 0);
        assert!((moves & (1u64 << 17)) != 0);
        
        // Verifica que ele não pula para fora do tabuleiro
        // (Isso já é garantido pelas nossas máscaras)
        assert_eq!(moves.count_ones(), 2, "Cavalo em a1 deve ter apenas 2 movimentos legais");
    }
}