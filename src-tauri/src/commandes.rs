use sqlx::SqlitePool;
use sqlx::{Column, Row};
use sqlx::AssertSqlSafe;

use crate::models::Produit;

#[tauri::command]
pub async fn get_produits(
    pool: tauri::State<'_, SqlitePool>,
) -> Result<Vec<Produit>, String> {
    sqlx::query_as::<_, Produit>(
        r#"
        SELECT id, nom, categorie, prix, quantite
        FROM produits
        ORDER BY id DESC
        "#,
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())
}

// VULNÉRABLE VOLONTAIREMENT — concaténation directe de l'entrée utilisateur
// dans la requête SQL, sans requête préparée. Ne JAMAIS faire ça en prod.
// AssertSqlSafe() contourne explicitement la protection anti-injection de sqlx 0.9+.
#[tauri::command]
pub async fn rechercher_produits(
    pool: tauri::State<'_, SqlitePool>,
    terme: String,
) -> Result<Vec<Produit>, String> {
    let query = format!(
        "SELECT id, nom, categorie, prix, quantite FROM produits WHERE nom LIKE '%{}%'",
        terme
    );

    sqlx::query_as::<_, Produit>(AssertSqlSafe(query))
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

// Endpoint séparé, retour en JSON générique (pas de typage strict FromRow),
// ce qui permet à un UNION SELECT de fonctionner même avec un nombre/type
// de colonnes différent de la table `produits`.
#[tauri::command]
pub async fn rechercher_produits_raw(
    pool: tauri::State<'_, SqlitePool>,
    terme: String,
) -> Result<Vec<serde_json::Value>, String> {
    let query = format!(
        "SELECT id, nom, categorie, prix, quantite FROM produits WHERE nom LIKE '%{}%'",
        terme
    );

    let rows = sqlx::query(AssertSqlSafe(query))
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    let result = rows
        .iter()
        .map(|row| {
            let mut obj = serde_json::Map::new();
            for (i, col) in row.columns().iter().enumerate() {
                let val: Option<String> = row.try_get(i).ok();
                obj.insert(
                    col.name().to_string(),
                    serde_json::Value::String(val.unwrap_or_default()),
                );
            }
            serde_json::Value::Object(obj)
        })
        .collect();

    Ok(result)
}