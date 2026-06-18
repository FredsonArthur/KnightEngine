# KnightEngine ♟️

**KnightEngine** é um motor de xadrez de alta performance desenvolvido em [Rust](https://www.rust-lang.org/). O objetivo principal deste projeto é explorar conceitos de engenharia de software e teoria dos jogos, utilizando estruturas de dados otimizadas para processamento eficiente de posições.

## 🚀 Sobre o Projeto

O motor é construído sobre o conceito de **Bitboards**, onde cada peça é representada por um inteiro de 64 bits (`u64`). Esta abordagem permite que a CPU avalie posições e gere movimentos legais através de operações bitwise, resultando em um desempenho extremamente elevado em comparação com representações tradicionais baseadas em arrays.

### Características Técnicas
- **Representação:** Bitboards de 64 bits (máscaras de bits para cada tipo de peça).
- **Linguagem:** Rust (foco em segurança de memória e concorrência).
- **Modularidade:** Código organizado em módulos para facilitar a implementação de algoritmos de busca e avaliação.

## 🛠️ Tecnologias Utilizadas

- **Linguagem:** Rust (estável)
- **Gerenciador de Pacotes:** Cargo
- **Arquitetura:** Bitboard-based Engine

## ⚙️ Instalação e Uso

Certifique-se de ter o [Rustup](https://rustup.rs/) instalado em seu sistema.

1. Clone o repositório:
   ```bash
   git clone [https://github.com/SEU_USUARIO/KnightEngine.git](https://github.com/SEU_USUARIO/KnightEngine.git)
   cd KnightEngine
   ```
2. Compile e execute o motot
    ```bash
    cargo run
    ```
## 📋 RoadMap de Desenvolvimento

    [x] Estrutura de dados básica (Bitboards)

    [x] Visualização do tabuleiro via terminal

    [ ] Gerador de movimentos legais (Move Generation)

    [ ] Avaliação estática de posição (Heurística)

    [ ] Algoritmo de busca (Minimax com poda Alpha-Beta)

    [ ] Suporte a protocolo UCI (Universal Chess Interface)

## 🤝 Contribuição

Este é um projeto de aprendizado e desenvolvimento contínuo. Sugestões de otimização, correções ou implementações de novas funcionalidades são bem-vindas!

## 📜 Licença

Este projeto está sob a licença MIT.