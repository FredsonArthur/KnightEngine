mod board;
mod moves;
mod engine;
mod tests;

use board::Board;
use engine::Engine;

fn main() {
    println!("--- KnightEngine: Motor de Xadrez ---");

    // 1. Inicializa o tabuleiro com as peças nas posições padrão
    let board = Board::new();

    // 2. Imprime o tabuleiro (usando a função que você já tem no board.rs)
    println!("Tabuleiro Inicial:");
    board.print_board();

    // 3. Verifica se as brancas estão em xeque (deve ser false no início)
    let white_in_check = Engine::is_in_check(board::Color::White, &board);
    println!("\nAs brancas estão em xeque? {}", white_in_check);

    // 4. Testa a geração de movimentos legais para o cavalo em g1 (índice 6)
    // O cavalo em g1 pode mover para f3 (índice 21) ou h3 (índice 23)
    let g1 = 1u64 << 6;
    let moves_g1 = Engine::get_legal_moves(g1, &board);

    println!("\nMovimentos legais para o Cavalo em g1:");
    Board::print_attacks(moves_g1);

    println!("\nTeste concluído com sucesso!");
}