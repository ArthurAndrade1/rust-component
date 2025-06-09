// Para rodar este exemplo:
// 1. Certifique-se de ter construído o projeto com `npm run build`
// 2. Execute `npm run example` ou `ts-node example.ts`

// Importa as funções do módulo Rust/Wasm localizado na pasta `pkg`
// O nome `rust_component` é derivado do nome do crate no Cargo.toml (saída padrão do wasm-pack)
import { add, greet, try_panic } from './pkg/rust_component';

// `wasm-pack` para o target `nodejs` geralmente não requer uma função `init` explícita
// se você usou `#[wasm_bindgen(start)]`. O módulo é inicializado na importação.
// Se você estivesse usando o target `web` e um bundler, poderia precisar de algo como:
// import init, { greet, add } from './pkg-web/rust_component';

async function runExample () {
  console.log("Executando exemplo TypeScript com módulo Rust/WASM...");

  // Chama a função `greet` do Rust
  const greeting = greet("Mundo TypeScript");
  console.log(`Mensagem do Rust: "${greeting}"`); // Esperado: "Olá do Rust, Mundo TypeScript!"

  // Chama a função `add` do Rust
  const sum = add(25, 17);
  console.log(`Soma do Rust: 25 + 17 = ${sum}`); // Esperado: 42

  // Testando o panic hook
  console.log("\nTentando uma função que causa pânico no Rust...");
  try {
    try_panic(); // Esta função deve causar um pânico no lado do Rust
    console.log("Pânico não ocorreu (isso não deveria acontecer).");
  } catch (e) {
    console.error("Pânico capturado no lado do TypeScript:", e);
    console.log("Se você vir um stack trace do Rust acima (graças ao console_error_panic_hook), está funcionando!");
  }

  console.log("\nExemplo concluído.");
}

// Executa a função de exemplo e captura quaisquer erros
runExample().catch(error => {
  console.error("Erro ao executar o exemplo:", error);
});
