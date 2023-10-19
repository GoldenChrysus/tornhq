use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub postgres: DatabaseConnection,
    pub jwt_secret: String,
}
