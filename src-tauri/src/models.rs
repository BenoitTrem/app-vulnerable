use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Produit {
    pub id: i64,
    pub nom: String,
    pub categorie: String,
    pub prix: f64,
    pub quantite: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub role: String,
}