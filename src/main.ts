import "./styles.css";
import { invoke } from "@tauri-apps/api/core";

interface Produit {
  id: number;
  nom: string;
  categorie: string;
  prix: number;
  quantite: number;
}

function formatPrix(value: unknown): string {
  const num = Number(value);
  return Number.isNaN(num) ? String(value) : `${num.toFixed(2)} $`;
}

function afficherProduits(produits: Produit[]) {
  const table = document.querySelector<HTMLTableSectionElement>(
    "#produits-table",
  );

  if (!table) return;

  table.innerHTML = produits
    .map(
      (produit) => `
        <tr>
          <td>${produit.id}</td>
          <td>${produit.nom}</td>
          <td>${produit.categorie}</td>
          <td>${formatPrix(produit.prix)}</td>
          <td>${produit.quantite}</td>
        </tr>
      `,
    )
    .join("");
}

function afficherStatistiques(produits: Produit[]) {
  // Nombre de produits différents
  const nombreProduits = produits.length;

  // Quantité totale en stock
  const quantiteEnStock = produits.reduce(
    (total, produit) => total + produit.quantite,
    0,
  );

  const totalElement = document.querySelector<HTMLElement>("#total-produits");
  const stockElement = document.querySelector<HTMLElement>("#total-stock");

  if (totalElement) {
    totalElement.textContent = nombreProduits.toString();
  }

  if (stockElement) {
    stockElement.textContent = quantiteEnStock.toString();
  }
}

async function chargerProduits() {
  try {
    const produits = await invoke<Produit[]>("get_produits");

    console.log("Produits reçus :", produits);

    afficherProduits(produits);
    afficherStatistiques(produits);
  } catch (error) {
    console.error("Erreur lors du chargement :", error);
  }
}

function attacherRecherche() {
  const inputRecherche = document.querySelector<HTMLInputElement>("#recherche");

  inputRecherche?.addEventListener("input", async () => {
    const terme = inputRecherche.value;
    try {
      const produits = await invoke<any[]>("rechercher_produits_raw", { terme });
      afficherProduits(produits);
    } catch (error) {
      console.error("Erreur recherche :", error);
    }
  });
}

const app = document.querySelector<HTMLDivElement>("#app");

if (app) {
  app.innerHTML = `
    <div class="app">

      <aside class="sidebar">
        <h1>Inventaire</h1>

        <nav>
          <button class="nav-button active">
            Tableau de bord
          </button>
        </nav>
      </aside>

      <main class="content">

        <header>
          <h2>Tableau de bord</h2>
          <p>Gestion de votre inventaire.</p>
        </header>

        <section class="stats">

          <div class="card">
            <span>Produits</span>
            <strong id="total-produits">0</strong>
          </div>

          <div class="card">
            <span>En stock</span>
            <strong id="total-stock">0</strong>
          </div>

        </section>

        <section class="inventory">

          <div class="section-header">
            <h3>Produits récents</h3>

            <input type="text" id="recherche" placeholder="Rechercher un produit..." />

            <button class="primary-button">
              Ajouter un produit
            </button>
          </div>

          <table>

            <thead>
              <tr>
                <th>ID</th>
                <th>Produit</th>
                <th>Catégorie</th>
                <th>Prix</th>
                <th>Quantité</th>
              </tr>
            </thead>

            <tbody id="produits-table"></tbody>

          </table>

        </section>

      </main>

    </div>
  `;

  attacherRecherche();
  chargerProduits();
}