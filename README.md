# Gerenciador de Tarefas em Rust

## 📜 Sobre

Este projeto é um gerenciador de tarefas construído em Rust. O objetivo é criar uma ferramenta CLI (Interface de Linha de Comando) para gerenciar tarefas, com planos futuros de expandir para WebAssembly. Este repositório demonstra a utilização de Rust para manipulação de dados, parsing de argumentos e serialização/deserialização de JSON.

## 🛠️ Arquitetura do Projeto

O projeto está estruturado da seguinte forma:

- **`src/`**: Contém o código-fonte do projeto.
  - **`main.rs`**: Ponto de entrada principal do programa. Gerencia o parsing dos argumentos, o carregamento e salvamento das tarefas, e a execução dos comandos.
  - **`cli.rs`**: Define a estrutura de parsing dos argumentos de linha de comando utilizando a biblioteca `clap`. Aqui são especificados os subcomandos e suas opções.
  - **`storage.rs`**: Gerencia a leitura e escrita dos dados de tarefas em um arquivo JSON. Utiliza `serde` e `serde_json` para serializar e desserializar os dados.
  - **`task.rs`**: Define a estrutura da tarefa e implementa a lógica necessária para a manipulação das tarefas.

- **`Cargo.toml`**: Arquivo de configuração do Cargo, que especifica as dependências e as configurações do projeto.

## 🗂️ Estrutura de Pastas

- **`src/`**
  - `main.rs`: Arquivo principal do aplicativo, onde a lógica de execução é implementada.
  - `cli.rs`: Configuração do CLI, definindo os comandos e suas opções.
  - `storage.rs`: Funções para carregar e salvar tarefas em um arquivo JSON.
  - `task.rs`: Definição da estrutura de dados `Task` e métodos associados.

- **`Cargo.toml`**: Configurações do projeto e dependências do Rust.

## 🛠️ Dependências

Este projeto utiliza as seguintes bibliotecas:

- **`clap`**: Facilita o parsing e validação de argumentos de linha de comando.
- **`serde`**: Biblioteca para serialização e desserialização de dados. Utiliza a feature `derive` para gerar código automaticamente.
- **`serde_json`**: Manipula dados JSON de forma eficiente, complementando `serde` para operações com JSON.

**`Comandos Disponíveis**`
**`add <DESCRIPTION>:**` Adiciona uma nova tarefa com uma descrição obrigatória. As opções --due_date e --priority são opcionais.
**`list:**` Lista todas as tarefas.

## 🚀 Como Executar

Para rodar o projeto, você precisa ter o Rust e o Cargo instalados. 

```bash
cargo run -- add "Minha primeira tarefa" --due-date "2024-07-30" --priority "high"
cargo run -- list

