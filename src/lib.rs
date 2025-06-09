mod utils;

use wasm_bindgen::prelude::*;

// Opcional: Quando a feature `wee_alloc` está habilitada, usa `wee_alloc` como alocador global.
// Isso pode reduzir o tamanho final do arquivo .wasm.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Importa a função `log` do namespace `console` do JavaScript.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Macro para facilitar o log no console do JavaScript a partir do Rust.
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Função executada quando o módulo Wasm é carregado (opcional).
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    utils::set_panic_hook(); // Configura panic hook para melhores mensagens de erro.
    console_log!("Módulo WebAssembly (Rust) inicializado.");
    Ok(())
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    console_log!("Função 'greet' chamada com nome: {}", name);
    format!("Olá do Rust, {}!", name)
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    console_log!("Função 'add' chamada com a: {}, b: {}", a, b);
    a + b
}

// Exemplo de uma função que pode falhar (para testar o panic hook)
#[wasm_bindgen]
pub fn try_panic() {
    panic!("Este é um pânico de teste do Rust!");
}
