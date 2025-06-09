# Rust Component para NPM com TypeScript

Este projeto é um template para criar um pacote NPM usando Rust compilado para WebAssembly (Wasm), com o objetivo de ser facilmente consumido por projetos TypeScript/JavaScript.

## Pré-requisitos

Antes de começar, certifique-se de ter o seguinte instalado:

1.  **Rust**: [Instalador do Rust](https://www.rust-lang.org/tools/install)
    - Verifique a instalação com `rustc --version`.
2.  **`wasm-pack`**: Ferramenta para construir e empacotar Rust para Wasm.
    - Instale com `cargo install wasm-pack`.
    - Verifique com `wasm-pack --version`.
3.  **Node.js e npm**: [Node.js](https://nodejs.org/) (que inclui npm).
    - Verifique com `node -v` e `npm -v`.
    - Recomendado Node.js >= 18.

## Configuração Inicial

1.  **Clone este repositório (ou use como template):**
    ```bash
    # git clone ...
    # cd rust-component
    ```
2.  **Atualize os placeholders:**
    - Em `Cargo.toml`: `authors`, `description`, `repository`.
    - Em `package.json`: `author`, `description`, `repository`, `bugs`, `homepage`.
3.  **Instale as dependências do Node.js:**
    ```bash
    npm install
    ```
    Isso instalará `typescript`, `ts-node`, `wasm-pack` (como dependência local) e outras ferramentas.

## Comandos Úteis

### Construindo o Pacote

- **Para desenvolvimento (Node.js target):**

  ```bash
  npm run build
  ```

  Isso compila o Rust para Wasm (sem otimizações de release) e gera os bindings na pasta `pkg/`. O target `nodejs` é ideal para uso em ambientes Node.js.

- **Para produção/release (Node.js target):**

  ```bash
  npm run build:release
  ```

  Compila com otimizações de release (menor tamanho, pode ser mais lento para compilar).

- **Para desenvolvimento (Web/Browser target):**

  ```bash
  npm run build:web
  ```

  Gera output na pasta `pkg-web/` otimizado para navegadores (e.g., usando bundlers como Webpack/Rollup).

- **Para produção/release (Web/Browser target):**
  ```bash
  npm run build:web-release
  ```

### Executando o Exemplo TypeScript

Após construir o pacote (e.g., `npm run build`), você pode rodar o exemplo:

```bash
npm run example
```

Isso usará `ts-node` para executar `example.ts`, que importa e usa as funções do módulo Rust/Wasm.

### Testes

- **Testes Rust:**

  ```bash
  cargo test --target wasm32-unknown-unknown
  ```

  (Se você tiver testes específicos para Wasm usando `wasm-bindgen-test`)

- **Testes TypeScript:** (Atualmente um placeholder)
  ```bash
  npm run test:ts
  ```
  Você precisará implementar testes para a integração TypeScript.

### Limpando Artefatos de Build

```bash
npm run clean
```

Remove as pastas `pkg/`, `pkg-web/` e `dist/`.

## Estrutura do Projeto

- `Cargo.toml`: Manifesto do projeto Rust.
- `src/`: Contém o código-fonte Rust.
  - `lib.rs`: Ponto de entrada principal da biblioteca Rust.
  - `utils.rs`: Módulo para utilitários (ex: `set_panic_hook`).
- `package.json`: Manifesto do pacote NPM.
- `tsconfig.json`: Configurações do compilador TypeScript.
- `example.ts`: Arquivo de exemplo mostrando como usar o módulo Wasm em TypeScript.
- `pkg/` (gerado): Contém o `.wasm` e os bindings JavaScript/TypeScript para target `nodejs`.
- `pkg-web/` (gerado): Contém o `.wasm` e os bindings para target `web`.
- `.gitignore`: Especifica arquivos e pastas a serem ignorados pelo Git.
- `README.md`: Este arquivo.

## Publicando no NPM

1.  **Construa para release:**
    ```bash
    npm run build:release
    ```
2.  **Faça login no NPM (se ainda não estiver logado):**
    ```bash
    npm login
    ```
3.  **Publique o pacote:**
    ```bash
    npm publish
    ```
    Certifique-se que a versão em `package.json` é única e que todos os metadados estão corretos.

## Usando o Pacote em Outro Projeto

Após publicar e instalar (`npm install rust-component` ou o nome que você deu), você pode importá-lo:

```typescript
import { greet, add } from 'rust-component' // ou o nome do seu pacote

async function myApp() {
  // wasm-pack gera um módulo que pode precisar ser inicializado (especialmente para target web)
  // Para target nodejs, geralmente funciona diretamente.
  // Se o seu wasm_bindgen(start) exporta uma função (como init), você pode precisar chamá-la.
  // Ex: await init(); // Se você exportou uma função init de wasm_bindgen(start)

  const message = greet('Mundo TypeScript')
  console.log(message) // "Olá do Rust, Mundo TypeScript!"

  const sum = add(10, 32)
  console.log(`10 + 32 = ${sum}`) // "10 + 32 = 42"
}

myApp()
```

## Wee Alloc (Opcional)

Para reduzir o tamanho do arquivo `.wasm`, você pode usar `wee_alloc`.

1.  Descomente as linhas relacionadas a `wee_alloc` em `Cargo.toml`.
2.  Descomente as linhas relacionadas a `wee_alloc` em `src/lib.rs`.
    Isso substituirá o alocador padrão por um mais leve, mas com algumas limitações.

Lembre-se de adaptar este template às suas necessidades específicas!
