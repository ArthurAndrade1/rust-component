pub fn set_panic_hook() {
    // Quando a feature `console_error_panic_hook` está habilitada, podemos chamar
    // a função `set_panic_hook` pelo menos uma vez durante a inicialização, e então
    // teremos mensagens de erro melhores se nosso código entrar em pânico.
    // Para mais detalhes, veja: https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
