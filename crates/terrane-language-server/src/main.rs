use tokio::io::{stdin, stdout};
use tower_lsp_server::{LspService, Server};

#[tokio::main]
async fn main() {
    let (service, socket) = LspService::build(terrane_language_server::Backend::new)
        .custom_method(
            "terrane/generatedRust",
            terrane_language_server::Backend::generated_rust,
        )
        .finish();
    Server::new(stdin(), stdout(), socket).serve(service).await;
}
