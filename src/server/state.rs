use super::db::DbPool;

/// Estado compartilhado entre todos os handlers -- injetado pelo axum via
/// `State<AppState>`. `#[derive(Clone)]` é barato aqui porque `DbPool` é
/// internamente um `Arc` (clonar o pool só incrementa um contador de
/// referências, não duplica as conexões).
#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub jwt_secret: String,
    /// Segredo global do servidor usado para derivar a chave de cifra de
    /// cada usuário (junto com o `encryption_salt` individual dele) --
    /// mesma ideia do `Encryption:Key` do backend original.
    pub encryption_secret: String,
}
