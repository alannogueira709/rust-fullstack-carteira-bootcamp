# 📊 CapitalFlow — Carteira de Investimentos Fullstack com Rust

<div align="center">

![Rust](https://img.shields.io/badge/Rust-2024%20Edition-black?style=for-the-badge&logo=rust)
![Axum](https://img.shields.io/badge/Axum-0.8-blue?style=for-the-badge&logo=rust)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-16-336791?style=for-the-badge&logo=postgresql)
![SQLx](https://img.shields.io/badge/SQLx-Async-orange?style=for-the-badge)
![Askama](https://img.shields.io/badge/Askama-Templates-purple?style=for-the-badge)
![TailwindCSS](https://img.shields.io/badge/Tailwind_CSS-Dark_Mode-38B2AC?style=for-the-badge&logo=tailwind-css)

**Aplicação web fullstack de alta performance para consolidação, gerenciamento e acompanhamento de carteiras de investimentos multi-ativos (Ações, Criptomoedas, Renda Fixa e Caixa).**

</div>

---

## 📌 1. Sobre o Projeto

O **CapitalFlow** é uma solução completa desenvolvida em **Rust** que unifica backend de alta performance, persistência assíncrona em banco de dados relacional (PostgreSQL), autenticação segura com cookies e tokens JWT assinados, e um frontend moderno com visual inspirado nas melhores plataformas de Wealth Management (Dark Mode, Glassmorphism, gráficos interativos e painéis analíticos).

O sistema permite que investidores cadastrem seus ativos, acompanhem a distribuição percentual da carteira por categoria, analisem rentabilidade histórica e métricas de risco, visualizem histórico de transações e realizem operações de cadastro, edição e exclusão de investimentos diretamente pela interface web ou via API REST.

---

## 🚀 2. Melhorias Implementadas (A, B, C, D e Frontend)

Partindo da estrutura base do desafio DIO, o projeto foi evoluído com todas as seguintes melhorias:

### 🌟 A. Dashboard Completo e Interativo (Askama + Tailwind)
- Substituição da rota textual por um **Dashboard completo** renderizado via SSR com [Askama](https://github.com/djc/askama).
- **4 Módulos Integrados**:
  1. **Overview**: Total Balance com gráfico de rentabilidade em área, Donut Chart de alocação de ativos e cards de maiores altas (*Top Gainers*) e atividades recentes.
  2. **My Portfolio (Assets)**: Tabela detalhada de ativos com busca em tempo real, filtros por categoria (*All Assets*, *Stocks*, *Crypto*, *Fixed Income*), badges de lucro/prejuízo (*P/L %*) e botões de ação.
  3. **Performance Analysis**: Análise de crescimento do patrimônio, comparação de *Alpha* contra Benchmarks (**S&P 500** e **CDI/Selic**), métricas de risco (*Volatilidade*, *Sharpe Ratio*, *Max Drawdown*, *Beta*) e barra de exposição setorial (*Tech*, *Financials*, *Crypto*, *Fixed Income*).
  4. **Transaction History**: Livro razão de transações de compra, venda e dividendos com métricas de volume e fluxo de caixa.

### 💰 B. Cálculo Automático do Patrimônio e Rentabilidade
- Adição dos campos de **quantidade (`quantity`)**, **preço médio (`avg_price`)**, **código (`ticker`)** e **categoria (`asset_type`)** para cada ativo.
- Cálculo em tempo real no backend do:
  - **Patrimônio Total da Carteira** ($\sum \text{quantidade} \times \text{preço atual}$)
  - **Total Investido / Custo de Aquisição** ($\sum \text{quantidade} \times \text{preço médio}$)
  - **Lucro / Prejuízo Absoluto e Percentual** ($P/L\%$)
  - **Distribuição Percentual de Alocação** (Ações vs Cripto vs Renda Fixa)

### ⚡ C. Operações CRUD Completas pela Interface Web
- **Cadastro de Ativos**: Modal interativo com validação de ticker, nome, categoria, quantidade, cotação atual e preço médio.
- **Edição em Tempo Real**: Modal pré-populado para ajuste rápido de cotações e quantidades.
- **Exclusão Segura**: Formulário com confirmação para deletar ativos.
- **Registro de Transações**: Modal para registrar novas compras, vendas e proventos.
- **Logout Seguro**: Limpeza do cookie HTTP-only e redirecionamento.

### 🔐 D. Multi-Tenancy e Isolamento por Usuário
- Vínculo relacional `user_id` na tabela `assets` e `transactions` com chave estrangeira `ON DELETE CASCADE`.
- Cada usuário autenticado acessa e gerencia exclusivamente a sua própria carteira.
- Ao cadastrar um novo usuário, uma carteira de demonstração inicial com dados ricos (AAPL, MSFT, BTC, TSLA, NVDA, CDI) é gerada automaticamente.

---

## 🛠️ 3. Tecnologias Utilizadas

| Tecnologia | Finalidade no Projeto |
| :--- | :--- |
| **[Rust](https://www.rust-lang.org/)** (Edição 2024) | Linguagem central de alta performance, concorrência e segurança de memória. |
| **[Axum](https://github.com/tokio-rs/axum)** (v0.8) | Framework web assíncrono baseado em Tokio e Tower. |
| **[Tokio](https://tokio.rs/)** | Runtime assíncrono multithread. |
| **[SQLx](https://github.com/launchbadge/sqlx)** | Driver SQL assíncrono com pool de conexões e mapeamento tipado (`FromRow`). |
| **[PostgreSQL](https://www.postgresql.org/)** (Docker) | Banco de dados relacional para persistência de dados. |
| **[Askama](https://github.com/djc/askama)** | Engine de templates HTML compile-time com tipagem estática e zero overhead. |
| **[JWT Simple](https://crates.io/crates/jwt-simple)** | Geração e validação de tokens JWT assinados criptograficamente com HS256. |
| **[Password Auth / Argon2](https://crates.io/crates/password-auth)** | Hash seguro de senhas com algoritmo de hashing resistente a ataques. |
| **[Axum Extra Cookies](https://crates.io/crates/axum-extra)** | Gerenciamento de cookies HTTP-Only no navegador. |
| **[TailwindCSS](https://tailwindcss.com/)** | Estilização moderna com design system dark-mode, glassmorphism e micro-interações. |

---

## ⚙️ 4. Como Executar a Aplicação

### Pré-requisitos
- [Rust & Cargo](https://www.rust-lang.org/tools/install) instalados.
- [Docker & Docker Compose](https://www.docker.com/) instalados e em execução.

### Passo 1: Clonar o Repositório
```bash
git clone https://github.com/SEU_USUARIO/rust-fullstack-carteira-investimentos.git
cd rust-fullstack-carteira-investimentos
```

### Passo 2: Iniciar o Banco de Dados com Docker
Suba o container do PostgreSQL em segundo plano:
```bash
docker compose up -d
```

### Passo 3: Executar a Aplicação
Execute o projeto com o Cargo:
```bash
cargo run
```

A aplicação executará automaticamente a checagem e criação das tabelas necessárias (`ensure_schema`) e inicializará o servidor na porta `3000`:
```text
INFO Starting service on http://localhost:3000
```

### Passo 4: Acessar a Aplicação
Abra o navegador em: **[http://localhost:3000](http://localhost:3000)**

> 💡 **Dica de Acesso**: Digite qualquer nome de usuário e senha na tela de login. Caso a conta ainda não exista, ela será criada automaticamente com uma carteira inicial de demonstração.

---

## 🧪 5. Como Testar as Funcionalidades

### 🖥️ Testes na Interface Web
1. **Login / Cadastro**:
   - Acesse `http://localhost:3000/login`, insira um usuário (ex: `investidor`) e uma senha.
2. **Dashboard Geral**:
   - Visualize o *Total Balance*, o gráfico em área de rentabilidade e o gráfico Donut de alocação.
3. **Gerenciar Ativos (Aba "Assets")**:
   - Clique em **"+ Add Asset"** e adicione um novo ativo (ex: `NVDA`, NVIDIA Corp, Categoria *Stocks*, Quantidade `10`, Preço Atual `$120.00`).
   - Use o campo de busca para filtrar ativos por nome ou ticker.
   - Filtre por categoria (*Crypto*, *Stocks*, *Fixed Income*).
   - Clique no ícone de lápis para editar a cotação de um ativo e veja o total recalculado imediatamente.
   - Clique no ícone de lixeira para remover um ativo.
4. **Análise de Performance (Aba "Performance")**:
   - Verifique os indicadores de Alpha contra o S&P 500 e CDI, além das métricas de Sharpe e Volatilidade.
5. **Histórico de Transações (Aba "Transactions")**:
   - Adicione uma transação com o botão **"+ Add Transaction"** e veja o livro razão atualizado.
6. **Logout**:
   - Clique no botão **"Sair"** na barra lateral para encerrar a sessão.

---

### 📡 Testes via API REST (JSON)

Você também pode testar os endpoints da API usando cURL, Postman ou Insomnia:

#### Listar Ativos
```bash
curl -X GET http://localhost:3000/api/assets
```

#### Obter Resumo Consolidado do Portfólio
```bash
curl -X GET http://localhost:3000/api/portfolio/summary
```

#### Criar Novo Ativo via API
```bash
curl -X POST http://localhost:3000/api/assets \
  -H "Content-Type: application/json" \
  -d '{
    "ticker": "PETR4",
    "name": "Petrobras PN",
    "asset_type": "Stocks",
    "quantity": 100.0,
    "unit_value": 38.50,
    "avg_price": 35.00
  }'
```

#### Atualizar Ativo
```bash
curl -X PATCH http://localhost:3000/api/assets \
  -H "Content-Type: application/json" \
  -d '{
    "id": 1,
    "unit_value": 185.00
  }'
```

#### Deletar Ativo
```bash
curl -X DELETE http://localhost:3000/api/assets/1
```

---

## 📂 6. Estrutura do Projeto

```text
├── Cargo.toml                  # Dependências e metadados do projeto
├── compose.yml                 # Definição do serviço PostgreSQL no Docker
├── .env                        # Variáveis de ambiente (DATABASE_URL)
├── migrations/                 # Scripts SQL de versionamento do banco de dados
│   ├── 20260328192535_create_assets.up.sql
│   ├── 20260329160020_create_users.up.sql
│   └── 20260330000000_update_schema_full.up.sql
├── src/
│   ├── main.rs                 # Ponto de entrada da aplicação
│   ├── app.rs                  # Inicialização do Axum, State e auto-migração
│   ├── error.rs                # Definição e tratamento centralizado de erros (AppError)
│   ├── models.rs               # Structs de dados (Asset, User, Transaction, PortfolioSummary)
│   ├── repository.rs           # Camada de acesso a dados SQLx com isolamento por usuário
│   ├── auth/
│   │   ├── mod.rs
│   │   ├── user.rs             # Autenticação JWT, cookies e registro com hash de senha
│   │   └── admin.rs            # Extractor para autenticação administrativa de API
│   └── routes/
│       ├── mod.rs
│       ├── api.rs              # Endpoints JSON RESTful (/api/assets, /api/portfolio)
│       └── frontend.rs         # Rotas HTML com templates Askama (/ , /login, /logout, formulários)
└── templates/
    ├── dashboard.html          # Template principal do Dashboard CapitalFlow
    └── login.html              # Template da tela de Login e Registro
```

---

## 🧠 7. Principais Aprendizados

Durante a realização deste desafio, foram consolidados os seguintes conceitos:

1. **Desenvolvimento Web Moderno com Rust & Axum**: Utilização de *Extractors* (`FromRequestParts`, `CookieJar`, `Json`, `Form`) para desacoplamento de responsabilidades e injeção de dependências via `AppState`.
2. **Persistência Assíncrona com SQLx**: Execução de queries assíncronas no PostgreSQL e mapeamento de tabelas para structs tipadas com segurança e performance.
3. **Autenticação Segura (JWT + Cookies HTTP-Only + Argon2)**: Implementação de fluxo completo de login e registro, protegendo contra vulnerabilidades de injeção e mantendo sessões de usuários isoladas.
4. **Server-Side Rendering (SSR) com Askama**: Renderização ultrarrápida de HTML compilado diretamente no binário Rust, sem custo de parsing em runtime.
5. **Arquitetura em Camadas**: Organização clara entre Modelos, Repositórios, Handlers e Templates, facilitando a escalabilidade do código.

---

<div align="center">
  <sub>Projeto desenvolvido com dedicação para o Bootcamp da <strong>Digital Innovation One (DIO)</strong>.</sub>
</div>
