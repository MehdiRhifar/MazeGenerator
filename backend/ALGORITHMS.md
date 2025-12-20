# Algorithmes de Génération de Labyrinthe

Ce document explique en détail le principe et le fonctionnement de chaque algorithme de génération de labyrinthe implémenté dans ce projet.

## Table des matières

1. [Recursive Backtracking (DFS)](#1-recursive-backtracking-dfs)
2. [Randomized Prim's Algorithm](#2-randomized-prims-algorithm)
3. [Kruskal's Algorithm](#3-kruskals-algorithm)
4. [Wilson's Algorithm (Loop-Erased Random Walk)](#4-wilsons-algorithm-loop-erased-random-walk)
5. [Recursive Division](#5-recursive-division)
6. [Comparaison des algorithmes](#6-comparaison-des-algorithmes)

---

## 1. Recursive Backtracking (DFS)

### Principe général

L'algorithme de backtracking (retour en arrière) utilise une approche de parcours en profondeur (Depth-First Search). Il explore le labyrinthe en creusant des passages le plus loin possible, puis revient en arrière quand il atteint une impasse.

### Fonctionnement étape par étape

1. **Initialisation** :
   - On commence avec une grille complètement remplie de murs
   - On choisit une cellule de départ aléatoire
   - On marque cette cellule comme visitée
   - On ajoute cette cellule à une pile (stack)

2. **Itération** (tant que la pile n'est pas vide) :
   - On prend la cellule au sommet de la pile (sans la retirer)
   - On cherche tous les voisins non visités de cette cellule
   - **Si des voisins non visités existent** :
     - On choisit un voisin aléatoire parmi eux
     - On retire le mur entre la cellule courante et ce voisin
     - On marque le voisin comme visité
     - On ajoute le voisin à la pile
   - **Si aucun voisin non visité n'existe** :
     - On dépile la cellule courante (backtracking)
     - On remonte dans la pile pour explorer d'autres branches

3. **Terminaison** :
   - L'algorithme se termine quand la pile est vide
   - Toutes les cellules ont été visitées

### Caractéristiques du labyrinthe généré

- **Chemins longs et sinueux** : L'algorithme a tendance à créer de longs couloirs
- **Peu de bifurcations** : Les chemins se divisent rarement
- **Rivière principale** : On observe souvent un long chemin principal avec quelques branches
- **Peu de cul-de-sacs** : Relativement peu d'impasses comparé à d'autres algorithmes

### Complexité

- **Temps** : O(n) où n est le nombre de cellules
- **Espace** : O(n) pour la pile dans le pire cas (grille en ligne)

### Implémentation

Voir `backend/src/generators/backtracking.rs`

```rust
// Structure principale
pub struct BacktrackingGenerator {
    stack: Vec<Point>,           // Pile pour le backtracking
    visited: Vec<bool>,          // Cellules visitées
    current_point: Option<Point>, // Position courante (pour animation)
    is_finished: bool,
    width: usize,
    height: usize,
}
```

---

## 2. Randomized Prim's Algorithm

### Principe général

L'algorithme de Prim randomisé est inspiré de l'algorithme de Prim pour les arbres couvrants minimaux. Il fonctionne par expansion progressive : on part d'une cellule et on grandit le labyrinthe en ajoutant aléatoirement des cellules adjacentes.

### Fonctionnement étape par étape

1. **Initialisation** :
   - On commence avec une grille complètement remplie de murs
   - On choisit une cellule de départ aléatoire
   - On marque cette cellule comme "dans le labyrinthe"
   - On ajoute tous les murs de cette cellule à une liste de "frontière"

2. **Définition de la frontière** :
   - La frontière contient tous les murs qui séparent :
     - Une cellule "dans le labyrinthe"
     - Une cellule "hors du labyrinthe"

3. **Itération** (tant que la frontière n'est pas vide) :
   - On choisit un mur **aléatoire** dans la frontière
   - On récupère la cellule voisine (celle qui est hors du labyrinthe)
   - **Si le voisin est toujours hors du labyrinthe** :
     - On retire le mur
     - On marque le voisin comme "dans le labyrinthe"
     - On ajoute tous les murs du voisin à la frontière
   - **Si le voisin est déjà dans le labyrinthe** :
     - On ignore ce mur (il serait redondant)
   - On retire le mur de la frontière

4. **Terminaison** :
   - L'algorithme se termine quand la frontière est vide
   - Toutes les cellules sont dans le labyrinthe

### Caractéristiques du labyrinthe généré

- **Chemins courts et nombreux** : Beaucoup de bifurcations
- **Aspect organique** : Le labyrinthe grandit de façon radiale depuis le centre
- **Nombreux cul-de-sacs** : Plus de branches mortes que le backtracking
- **Distribution uniforme** : Les passages sont répartis de façon équilibrée

### Complexité

- **Temps** : O(n log n) à cause de la sélection aléatoire dans la frontière
- **Espace** : O(n) pour stocker la frontière et l'état des cellules

### Implémentation

Voir `backend/src/generators/prim.rs`

```rust
// Structure d'un mur de frontière
struct Wall {
    x: usize,
    y: usize,
    wall_type: WallType,
    neighbor_x: usize,  // Position du voisin
    neighbor_y: usize,
}

// Structure principale
pub struct PrimGenerator {
    in_maze: Vec<bool>,          // Cellules dans le labyrinthe
    frontier_walls: Vec<Wall>,   // Liste de frontière
    current_point: Option<Point>,
    is_finished: bool,
    width: usize,
    height: usize,
}
```

**Point clé** : Utilisation de `swap_remove()` pour retirer efficacement un élément aléatoire de la frontière en O(1).

---

## 3. Kruskal's Algorithm

### Principe général

L'algorithme de Kruskal est un algorithme classique pour trouver l'arbre couvrant minimal d'un graphe. Appliqué aux labyrinthes, il traite tous les murs dans un ordre aléatoire et retire ceux qui ne créent pas de cycle.

### Fonctionnement étape par étape

1. **Initialisation** :
   - On commence avec une grille complètement remplie de murs
   - Chaque cellule est son propre "ensemble" (composante connexe)
   - On crée une liste de **tous les murs possibles** de la grille
   - On **mélange cette liste aléatoirement**

2. **Structure Union-Find** :
   - On utilise une structure de données Union-Find pour tracker les ensembles de cellules connectées
   - Deux opérations principales :
     - `find(cellule)` : trouve l'ensemble auquel appartient une cellule
     - `union(cellule1, cellule2)` : fusionne deux ensembles

3. **Itération** (pour chaque mur dans la liste mélangée) :
   - On prend le mur courant
   - On identifie les deux cellules de part et d'autre du mur
   - On vérifie si ces deux cellules appartiennent au **même ensemble** :
     - **Si ensembles différents** :
       - On retire le mur (créer un passage)
       - On fusionne les deux ensembles (union)
     - **Si même ensemble** :
       - On garde le mur (sinon on créerait une boucle)

4. **Terminaison** :
   - L'algorithme se termine quand tous les murs ont été traités
   - Toutes les cellules appartiennent au même ensemble

### Caractéristiques du labyrinthe généré

- **Très aléatoire** : Aucune structure prévisible
- **Distribution uniforme** : Tous les labyrinthes possibles ont la même probabilité
- **Nombreuses branches courtes** : Beaucoup de petites bifurcations
- **Aspect "fractal"** : Ressemble à un réseau complexe

### Complexité

- **Temps** : O(n log n) où n est le nombre de murs
  - Le tri/mélange est en O(n log n)
  - Union-Find avec compression de chemin et union by rank est quasi O(1) par opération
- **Espace** : O(n) pour la liste des murs et la structure Union-Find

### Implémentation

Voir `backend/src/generators/kruskal.rs`

```rust
pub struct KruskalGenerator {
    // Union-Find : parent[i] = parent de la cellule i (-1 si racine)
    parent: Vec<isize>,

    // Rang pour l'optimisation (hauteur de l'arbre)
    rank: Vec<usize>,

    // Liste de tous les murs, mélangée aléatoirement
    walls: Vec<WallEntry>,

    current_wall_index: usize,
    current_point: Option<Point>,
    is_finished: bool,
    width: usize,
    height: usize,
}
```

**Optimisations Union-Find** :
1. **Compression de chemin** : Lors d'un `find()`, on met à jour les parents pour pointer directement vers la racine
2. **Union by rank** : On attache toujours l'arbre le moins profond sous l'arbre le plus profond

```rust
// Trouver la racine avec compression de chemin
fn find(&mut self, cell: usize) -> usize {
    if self.parent[cell] < 0 {
        return cell;  // Racine trouvée
    }
    // Compression : mettre à jour le parent pour pointer vers la racine
    let root = self.find(self.parent[cell] as usize);
    self.parent[cell] = root as isize;
    root
}

// Unir deux ensembles avec union by rank
fn union(&mut self, cell1: usize, cell2: usize) -> bool {
    let root1 = self.find(cell1);
    let root2 = self.find(cell2);

    if root1 == root2 {
        return false;  // Déjà dans le même ensemble
    }

    // Attacher l'arbre de rang inférieur sous celui de rang supérieur
    if self.rank[root1] < self.rank[root2] {
        self.parent[root1] = root2 as isize;
    } else if self.rank[root1] > self.rank[root2] {
        self.parent[root2] = root1 as isize;
    } else {
        self.parent[root2] = root1 as isize;
        self.rank[root1] += 1;  // Augmenter le rang si égalité
    }

    true
}
```

---

## 4. Wilson's Algorithm (Loop-Erased Random Walk)

### Principe général

L'algorithme de Wilson est l'un des plus élégants mathématiquement. Il utilise des **marches aléatoires avec effacement de boucles** (loop-erased random walk). C'est le seul algorithme qui génère des labyrinthes avec une distribution **parfaitement uniforme** : tous les labyrinthes possibles ont exactement la même probabilité.

### Fonctionnement étape par étape

1. **Initialisation** :
   - On commence avec une grille complètement remplie de murs
   - On choisit une cellule aléatoire et on la marque comme "dans le labyrinthe"
   - Toutes les autres cellules sont "hors du labyrinthe"

2. **Boucle principale** (tant qu'il reste des cellules hors du labyrinthe) :

   **Phase 1 : Choisir un point de départ**
   - On choisit aléatoirement une cellule qui n'est pas encore dans le labyrinthe
   - Cette cellule sera le départ de notre marche aléatoire

   **Phase 2 : Marche aléatoire (Walking)**
   - À partir du point de départ, on effectue une marche aléatoire :
     - On choisit un voisin aléatoire
     - On enregistre la direction prise dans un tableau `path`
     - **Si on arrive sur une cellule déjà visitée dans cette marche** :
       - On a créé une boucle !
       - On **efface** toute la boucle du chemin
       - Cela s'appelle "loop erasure"
     - On continue jusqu'à atteindre une cellule "dans le labyrinthe"

   **Phase 3 : Carving (Creuser le chemin)**
   - On suit le chemin enregistré depuis le point de départ
   - Pour chaque étape du chemin :
     - On ajoute la cellule au labyrinthe
     - On retire le mur entre cette cellule et la suivante
   - On continue jusqu'à rejoindre le labyrinthe

3. **Terminaison** :
   - L'algorithme se termine quand toutes les cellules sont dans le labyrinthe

### Exemple visuel de loop erasure

```
Marche aléatoire :
A → B → C → D → E → C

On est revenu en C ! Il y a une boucle : C → D → E → C

Après effacement de la boucle :
A → B → C

On continue la marche depuis C (avec une nouvelle direction)
```

### Caractéristiques du labyrinthe généré

- **Distribution parfaitement uniforme** : Propriété mathématique unique
- **Pas de biais** : Contrairement aux autres algorithmes
- **Aspect très aléatoire** : Aucune structure prévisible
- **Lent au début** : Les premières marches peuvent être longues car le labyrinthe est petit

### Complexité

- **Temps** : O(n³) dans le pire cas théorique, mais O(n log n) en moyenne
  - Les marches aléatoires peuvent être longues
  - Mais en pratique, l'algorithme converge rapidement
- **Espace** : O(n) pour stocker le chemin et l'état des cellules

### Implémentation

Voir `backend/src/generators/wilson.rs`

```rust
// Machine à états pour Wilson
#[derive(Debug, Clone, Copy, PartialEq)]
enum WilsonState {
    PickingStart,   // Choisir une cellule de départ
    Walking,        // Marche aléatoire en cours
    CarvingPath,    // Creuser le chemin trouvé
    Finished,       // Terminé
}

pub struct WilsonGenerator {
    in_maze: Vec<bool>,                // Cellules dans le labyrinthe
    path: Vec<Option<Direction>>,      // Direction pour aller à la prochaine cellule
    remaining_cells: Vec<Point>,       // Cellules pas encore visitées
    walk_start: Option<Point>,         // Début de la marche courante
    current_position: Option<Point>,   // Position courante
    state: WilsonState,                // État de la machine
    width: usize,
    height: usize,
}
```

**Point clé** : L'effacement de boucles se fait en suivant le tableau `path` et en réinitialisant les directions à `None`.

```rust
fn erase_loop(&mut self, grid: &MazeGrid, loop_start: Point) {
    let mut current = loop_start;

    loop {
        let current_index = grid.get_index(current.x, current.y);
        let direction = self.path[current_index];

        // Effacer cette étape
        self.path[current_index] = None;

        if let Some(dir) = direction {
            // Avancer dans la direction
            current = /* calculer la prochaine position */;

            // Si on revient au point de départ de la boucle, arrêter
            if current == loop_start {
                break;
            }
        } else {
            break;
        }
    }
}
```

---

## 5. Recursive Division

### Principe général

L'algorithme de division récursive est **fondamentalement différent** de tous les autres : c'est une approche **top-down** (de haut en bas). Au lieu de commencer avec une grille pleine de murs et de les retirer, on commence avec une **grille vide** et on **ajoute des murs**.

### Fonctionnement étape par étape

1. **Initialisation** :
   - On commence avec une grille **complètement vide** (aucun mur)
   - La grille entière est considérée comme une seule "chambre"
   - On ajoute cette chambre à une pile de chambres à traiter

2. **Itération** (tant que la pile n'est pas vide) :

   **Étape 1 : Prendre une chambre**
   - On dépile une chambre à diviser

   **Étape 2 : Vérifier si on peut diviser**
   - On vérifie si la chambre est assez grande pour être divisée :
     - Largeur ≥ 2 cellules pour division verticale
     - Hauteur ≥ 2 cellules pour division horizontale
   - Si trop petite, on passe à la chambre suivante

   **Étape 3 : Choisir l'orientation**
   - On décide si on divise horizontalement ou verticalement
   - **Avec un biais** basé sur les proportions :
     - Si la chambre est plus large que haute : favoriser division verticale
     - Si la chambre est plus haute que large : favoriser division horizontale
     - Si carrée : 50/50

   **Étape 4 : Tracer le mur**
   - On choisit aléatoirement où placer le mur
   - On choisit aléatoirement où placer le **passage** (une ouverture)
   - On ajoute le mur partout **sauf** au passage

   **Étape 5 : Créer les sous-chambres**
   - Le mur divise la chambre en deux sous-chambres
   - On ajoute ces deux sous-chambres à la pile

3. **Terminaison** :
   - L'algorithme se termine quand la pile est vide
   - Toutes les chambres ont été divisées jusqu'à être trop petites

### Exemple visuel

```
Étape 1 : Grille vide (une seule chambre)
┌─────────────┐
│             │
│             │
│             │
│             │
└─────────────┘

Étape 2 : Division horizontale avec passage à droite
┌─────────────┐
│             │
├─────────────┤  ← Mur avec passage
│             │
│             │
└─────────────┘

Étape 3 : Division verticale de la chambre du haut
┌─────┬───────┐
│     │       │
├─────┴───────┤
│             │
│             │
└─────────────┘

... et ainsi de suite
```

### Caractéristiques du labyrinthe généré

- **Structure hiérarchique** : On voit clairement les grandes divisions
- **Longs couloirs** : Les murs créent naturellement de longs passages droits
- **Apparence "construite"** : Moins organique que les autres algorithmes
- **Régions distinctes** : On peut identifier les zones créées par les premières divisions
- **Peu de petits cul-de-sacs** : Les impasses sont plutôt de longs couloirs

### Complexité

- **Temps** : O(n) où n est le nombre de cellules
  - Chaque cellule est visitée une seule fois
- **Espace** : O(log n) pour la pile de chambres (profondeur de récursion)

### Implémentation

Voir `backend/src/generators/recursive_division.rs`

```rust
// Structure représentant une chambre
struct Chamber {
    x_min: usize,
    y_min: usize,
    x_max: usize,  // Exclusif
    y_max: usize,  // Exclusif
}

impl Chamber {
    fn width(&self) -> usize { self.x_max - self.x_min }
    fn height(&self) -> usize { self.y_max - self.y_min }
    fn can_divide_horizontally(&self) -> bool { self.height() >= 2 }
    fn can_divide_vertically(&self) -> bool { self.width() >= 2 }
}

pub struct RecursiveDivisionGenerator {
    chambers: Vec<Chamber>,        // Pile des chambres à diviser
    current_point: Option<Point>,  // Position courante (pour animation)
    is_finished: bool,
    width: usize,
    height: usize,
}
```

**Point crucial** : L'initialisation commence avec une grille vide !

```rust
fn start(&mut self, grid: &mut MazeGrid) {
    // IMPORTANT : Commencer avec une grille VIDE (pas de murs)
    grid.clear_grid();

    // Ajouter la chambre initiale (toute la grille)
    self.chambers = vec![Chamber {
        x_min: 0,
        y_min: 0,
        x_max: self.width,
        y_max: self.height,
    }];
}
```

**Biais d'orientation** pour des labyrinthes plus naturels :

```rust
let divide_horizontally = if can_horizontal && can_vertical {
    if chamber.width() > chamber.height() {
        rng.random_range(0..4) == 0  // 25% horizontal si plus large
    } else if chamber.height() > chamber.width() {
        rng.random_range(0..4) != 0  // 75% horizontal si plus haut
    } else {
        rng.random_range(0..2) == 0  // 50/50 si carré
    }
} else {
    can_horizontal
};
```

---

## 6. Comparaison des algorithmes

### Tableau récapitulatif

| Algorithme | Approche | Distribution | Vitesse | Chemins | Utilisation mémoire |
|-----------|----------|--------------|---------|---------|---------------------|
| **Backtracking** | Bottom-up, DFS | Biaisée | Rapide | Longs et sinueux | Moyenne (pile) |
| **Prim** | Bottom-up, expansion | Biaisée | Moyenne | Courts, nombreuses branches | Moyenne (frontière) |
| **Kruskal** | Bottom-up, ensembles | Biaisée | Rapide | Très aléatoires | Élevée (tous les murs) |
| **Wilson** | Bottom-up, marches aléatoires | **Uniforme** | Lente au début | Très aléatoires | Moyenne |
| **Recursive Division** | **Top-down**, division | Biaisée | Rapide | Longs couloirs | Faible (pile) |

### Quand utiliser quel algorithme ?

**Backtracking (DFS)**
- ✅ Génération rapide
- ✅ Animation fluide et compréhensible
- ✅ Bon pour des labyrinthes avec un "chemin principal"
- ❌ Pas très varié visuellement

**Prim**
- ✅ Aspect organique, naturel
- ✅ Bonne balance entre aléatoire et structure
- ✅ Animation intéressante (expansion progressive)
- ❌ Peut créer beaucoup de cul-de-sacs courts

**Kruskal**
- ✅ Très rapide
- ✅ Très aléatoire
- ✅ Bon pour de grandes grilles
- ❌ Animation moins intéressante (pas d'ordre spatial)
- ❌ Consomme plus de mémoire

**Wilson**
- ✅ **Seul algorithme parfaitement uniforme**
- ✅ Garanties mathématiques fortes
- ✅ Animation unique (marches avec effacement)
- ❌ Peut être lent au début
- ❌ Animation parfois chaotique

**Recursive Division**
- ✅ Très rapide
- ✅ Structure hiérarchique claire
- ✅ Longs couloirs (bon pour certains jeux)
- ✅ Faible consommation mémoire
- ❌ Aspect "artificiel"
- ❌ Prévisible (on voit les grandes divisions)

### Propriétés mathématiques

**Arbre couvrant (Spanning Tree)**
Tous ces algorithmes génèrent un **arbre couvrant** du graphe de la grille :
- Connexe : Toutes les cellules sont accessibles
- Acyclique : Aucune boucle (un seul chemin entre deux points)
- n-1 arêtes pour n sommets

**Distribution uniforme**
- **Wilson uniquement** : Distribution parfaitement uniforme
- **Autres algorithmes** : Biaisés vers certaines structures
  - Backtracking : Biais vers longs chemins
  - Prim/Kruskal : Biais vers chemins courts
  - Recursive Division : Biais vers structure hiérarchique

### Complexité comparée

| Algorithme | Complexité temporelle | Complexité spatiale |
|-----------|----------------------|---------------------|
| Backtracking | O(n) | O(n) pire cas |
| Prim | O(n log n) | O(n) |
| Kruskal | O(n log n) | O(n) |
| Wilson | O(n³) pire cas, O(n log n) moyenne | O(n) |
| Recursive Division | O(n) | O(log n) |

---

## Convention des murs

Dans notre implémentation :

- **Mur vertical** : Situé à **droite** de la cellule (x, y)
  - Sépare (x, y) et (x+1, y)

- **Mur horizontal** : Situé en **bas** de la cellule (x, y)
  - Sépare (x, y) et (x, y+1)

Cette convention permet de représenter tous les murs internes de la grille sans duplication.

## Structures de données communes

Tous les générateurs implémentent le trait `GenerationAlgorithm` :

```rust
pub trait GenerationAlgorithm {
    fn start(&mut self, grid: &mut MazeGrid);
    fn step(&mut self, grid: &mut MazeGrid) -> (GenerationResult, Vec<WallChange>);
    fn is_finished(&self) -> bool;
    fn get_current_position(&self) -> Option<Point>;
    fn get_name(&self) -> &'static str;
}
```

Cette architecture permet :
- Une interface uniforme pour tous les algorithmes
- Une animation pas à pas via la méthode `step()`
- Un suivi de la position courante pour l'animation visuelle
