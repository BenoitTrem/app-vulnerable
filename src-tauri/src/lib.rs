mod commandes;
mod db;
mod models;

use db::init_db;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();

            tauri::async_runtime::block_on(async move {
                let pool = init_db(&handle)
                    .await
                    .expect("Impossible d'initialiser la base de données");

                handle.manage(pool);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commandes::get_produits,
            commandes::rechercher_produits,
            commandes::rechercher_produits_raw,
        ])
        .run(tauri::generate_context!())
        .expect("Erreur lors du démarrage de Tauri");
}