use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::fs;
use tauri::{AppHandle, Manager};

pub async fn init_db(app: &AppHandle) -> Result<SqlitePool, sqlx::Error> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| {
            sqlx::Error::Io(std::io::Error::other(e.to_string()))
        })?;

    println!("Dossier de données : {}", app_data_dir.display());

    // Crée le dossier s'il n'existe pas
    fs::create_dir_all(&app_data_dir)
        .map_err(sqlx::Error::Io)?;

    let database_path = app_data_dir.join("inventaire.db");

    println!("Base de données : {}", database_path.display());

    // Crée explicitement le fichier SQLite
    if !database_path.exists() {
        fs::File::create(&database_path)
            .map_err(sqlx::Error::Io)?;

        println!("Fichier SQLite créé.");
    }

    let database_url = format!(
        "sqlite://{}",
        database_path
            .to_string_lossy()
            .replace('\\', "/")
    );

    println!("URL SQLite : {}", database_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS produits (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        nom TEXT NOT NULL,
        categorie TEXT NOT NULL,
        prix REAL NOT NULL,
        quantite INTEGER NOT NULL
    )
    "#,
)
.execute(&pool)
.await?;

sqlx::query(
    r#"
    INSERT INTO produits (nom, categorie, prix, quantite)
    VALUES (?, ?, ?, ?)
    "#,
)
.bind("Clavier mécanique")
.bind("Électronique")
.bind(89.99)
.bind(25)
.execute(&pool)
.await?;

sqlx::query(
    r#"
    INSERT INTO produits (nom, categorie, prix, quantite)
    VALUES (?, ?, ?, ?)
    "#,
)
.bind("Souris sans fil")
.bind("Électronique")
.bind(39.99)
.bind(42)
.execute(&pool)
.await?;

sqlx::query(
    r#"
    INSERT INTO produits (nom, categorie, prix, quantite)
    VALUES (?, ?, ?, ?)
    "#,
)
.bind("Écran 24 pouces")
.bind("Électronique")
.bind(199.99)
.bind(8)
.execute(&pool)
.await?;

sqlx::query(
    r#"
    INSERT INTO produits (nom, categorie, prix, quantite)
    VALUES (?, ?, ?, ?)
    "#,
)
.bind("Chaise de bureau")
.bind("Mobilier")
.bind(149.99)
.bind(15)
.execute(&pool)
.await?;

sqlx::query(
    r#"
    INSERT INTO produits (nom, categorie, prix, quantite)
    VALUES (?, ?, ?, ?)
    "#,
)
.bind("Bureau en bois")
.bind("Mobilier")
.bind(299.99)
.bind(6)
.execute(&pool)
.await?;

sqlx::query(
    r#"
    INSERT INTO produits (nom, categorie, prix, quantite)
    VALUES (?, ?, ?, ?)
    "#,
)
.bind("Casque audio")
.bind("Audio")
.bind(79.99)
.bind(12)
.execute(&pool)
.await?;

sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        username TEXT NOT NULL,
        password TEXT NOT NULL,
        role TEXT NOT NULL
    )
    "#,
)
.execute(&pool)
.await?;

sqlx::query(
    r#"INSERT INTO users (username, password, role) VALUES (?, ?, ?)"#,
)
.bind("admin")
.bind("SuperSecret123!")
.bind("admin")
.execute(&pool)
.await?;

sqlx::query(
    r#"INSERT INTO users (username, password, role) VALUES (?, ?, ?)"#,
)
.bind("jdupont")
.bind("motdepasse1")
.bind("employe")
.execute(&pool)
.await?;

sqlx::query(
    r#"INSERT INTO users (username, password, role) VALUES (?, ?, ?)"#,
)
.bind("mtremblay")
.bind("qwerty2024")
.bind("employe")
.execute(&pool)
.await?;


println!("Base de données initialisée.");

Ok(pool)

}
