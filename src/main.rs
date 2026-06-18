/// O módulo `board` contém a estrutura do tabuleiro e os métodos de Bitboard.
mod board;

/// O módulo `moves` contém a lógica de geração de ataques para todas as peças.
mod moves;

/// O módulo `tests` contém a suíte de testes unitários para validar a lógica das peças.
/// O atributo `#[cfg(test)]` garante que este código seja compilado apenas durante os testes.
mod tests; 

fn main() {
    // Ponto de entrada da aplicação.
    // Inicializa a Engine e prepara o ambiente para o loop principal.
    println!("KnightEngine inicializado com sucesso.");
    
    // TODO: No futuro, implementaremos aqui o loop principal de jogo (Game Loop),
    // a leitura de comandos UCI (Universal Chess Interface) e a busca de lances.
}