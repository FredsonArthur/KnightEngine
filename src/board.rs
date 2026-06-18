/// Estrutura que representa o estado do tabuleiro de xadrez usando Bitboards.
/// Cada peça é representada por um `u64`, onde cada bit indica a presença (1) ou ausência (0) da peça.
pub struct Board {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,
    
    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,
}

impl Board {
    /// Cria um novo tabuleiro configurado com as posições iniciais padrão do xadrez.
    pub fn new() -> Self {
        Self {
            // Rank 2 para brancas (bits 8-15)
            white_pawns: 0xFF00,
            white_knights: 0x42,
            white_bishops: 0x24,
            white_rooks: 0x81,
            white_queens: 0x8,
            white_king: 0x10,
            
            // Rank 7 para pretas (bits 48-55)
            black_pawns: 0xFF000000000000,
            black_knights: 0x4200000000000000,
            black_bishops: 0x2400000000000000,
            black_rooks: 0x8100000000000000,
            black_queens: 0x800000000000000,
            black_king: 0x1000000000000000,
        }
    }

    /// Exibe o tabuleiro no console, percorrendo os ranks do 7 ao 0 para visualização correta.
    /// Utiliza símbolos ASCII para representar cada tipo de peça.
    pub fn print_board(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = 1u64 << (rank * 8 + file);
                // Determina qual peça ocupa a casa atual, se houver
                let symbol = if (self.white_pawns & square) != 0 { 'P' }
                else if (self.white_knights & square) != 0 { 'N' }
                else if (self.white_bishops & square) != 0 { 'B' }
                else if (self.white_rooks & square) != 0 { 'R' }
                else if (self.white_queens & square) != 0 { 'Q' }
                else if (self.white_king & square) != 0 { 'K' }
                else if (self.black_pawns & square) != 0 { 'p' }
                else if (self.black_knights & square) != 0 { 'n' }
                else if (self.black_bishops & square) != 0 { 'b' }
                else if (self.black_rooks & square) != 0 { 'r' }
                else if (self.black_queens & square) != 0 { 'q' }
                else if (self.black_king & square) != 0 { 'k' }
                else { '.' }; // Casa vazia
                print!(" {} ", symbol);
            }
            println!();
        }
    }

    /// Desenha graficamente as casas atacadas (representadas por 'x') em um bitboard.
    pub fn print_attacks(attacks: u64) {
        println!("\nVisualização de Ataques:");
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square_index = rank * 8 + file;
                let square = 1u64 << square_index;
                
                // Se o bit estiver setado (1), imprime 'x', caso contrário '.'
                if (attacks & square) != 0 {
                    print!(" x ");
                } else {
                    print!(" . ");
                }
            }
            println!();
        }
    }

    /// Retorna um Bitboard contendo todas as peças presentes no tabuleiro.
    /// Útil para calcular o `occupancy` necessário para peças deslizantes.
    pub fn get_all_pieces(&self) -> u64 {
        self.white_pawns | self.white_knights | self.white_bishops | 
        self.white_rooks | self.white_queens | self.white_king |
        self.black_pawns | self.black_knights | self.black_bishops | 
        self.black_rooks | self.black_queens | self.black_king
    }
}