use super::{GenerationAlgorithm, GenerationResult, WallChange, WallType};
use crate::maze_grid::MazeGrid;
use crate::Point;
use rand::seq::SliceRandom;

struct WallEntry {
    x: usize,
    y: usize,
    wall_type: WallType,
}

#[derive(Default)]
pub struct KruskalGenerator {
    // Union-Find pour tracker les ensembles de cellules connectées
    // parent[i] = parent de la cellule i (-1 si racine)
    parent: Vec<isize>,

    // Rang pour l'optimisation Union-Find (hauteur de l'arbre)
    rank: Vec<usize>,

    // Liste de tous les murs à traiter
    walls: Vec<WallEntry>,

    // Index du mur courant à traiter
    current_wall_index: usize,

    // Position courante pour l'animation
    current_point: Option<Point>,

    // État de fin
    is_finished: bool,

    // Dimensions
    width: usize,
    height: usize,
}

impl KruskalGenerator {

    // Trouver la racine de l'ensemble contenant la cellule (avec compression de chemin)
    fn find(&mut self, cell: usize) -> usize {
        if self.parent[cell] < 0 {
            return cell;
        }

        // Compression de chemin pour optimisation
        let root = self.find(self.parent[cell] as usize);
        self.parent[cell] = root as isize;
        root
    }

    // Unir deux ensembles (union by rank)
    fn union(&mut self, cell1: usize, cell2: usize) -> bool {
        let root1 = self.find(cell1);
        let root2 = self.find(cell2);

        if root1 == root2 {
            return false; // Déjà dans le même ensemble
        }

        // Union by rank : attacher l'arbre de rang inférieur sous celui de rang supérieur
        if self.rank[root1] < self.rank[root2] {
            self.parent[root1] = root2 as isize;
        } else if self.rank[root1] > self.rank[root2] {
            self.parent[root2] = root1 as isize;
        } else {
            self.parent[root2] = root1 as isize;
            self.rank[root1] += 1;
        }

        true
    }
}

impl GenerationAlgorithm for KruskalGenerator {
    fn start(&mut self, grid: &mut MazeGrid) {
        self.width = grid.width;
        self.height = grid.height;
        let total_cells = self.width * self.height;

        // Initialiser Union-Find : chaque cellule est sa propre racine
        self.parent = vec![-1; total_cells];
        self.rank = vec![0; total_cells];

        // Remplir la grille de murs
        grid.fill_grid();

        // Créer la liste de tous les murs possibles
        self.walls = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                // Mur vertical (à droite)
                if x < self.width - 1 {
                    self.walls.push(WallEntry {
                        x,
                        y,
                        wall_type: WallType::Vertical,
                    });
                }

                // Mur horizontal (en bas)
                if y < self.height - 1 {
                    self.walls.push(WallEntry {
                        x,
                        y,
                        wall_type: WallType::Horizontal,
                    });
                }
            }
        }

        // Mélanger aléatoirement la liste des murs
        let mut rng = rand::rng();
        self.walls.shuffle(&mut rng);

        self.current_wall_index = 0;
        self.is_finished = false;
        self.current_point = Some(Point { x: 0, y: 0 });
    }

    fn step(&mut self, grid: &mut MazeGrid) -> (GenerationResult, Vec<WallChange>) {
        // Si tous les murs ont été traités, terminé
        if self.current_wall_index >= self.walls.len() {
            self.is_finished = true;
            self.current_point = None;
            return (GenerationResult::Finished, Vec::new());
        }

        // Prendre le mur courant et copier ses valeurs
        let wall_x = self.walls[self.current_wall_index].x;
        let wall_y = self.walls[self.current_wall_index].y;
        let wall_type = self.walls[self.current_wall_index].wall_type;
        self.current_wall_index += 1;

        // Déterminer les deux cellules de part et d'autre du mur
        let (cell1_x, cell1_y, cell2_x, cell2_y) = match wall_type {
            WallType::Vertical => {
                // Mur vertical à droite de (x, y)
                (wall_x, wall_y, wall_x + 1, wall_y)
            }
            WallType::Horizontal => {
                // Mur horizontal en bas de (x, y)
                (wall_x, wall_y, wall_x, wall_y + 1)
            }
        };

        let cell1_index = grid.get_index(cell1_x, cell1_y);
        let cell2_index = grid.get_index(cell2_x, cell2_y);

        // Mettre à jour la position courante pour l'animation
        self.current_point = Some(Point { x: cell1_x, y: cell1_y });

        // Vérifier si les deux cellules sont dans des ensembles différents
        if self.find(cell1_index) != self.find(cell2_index) {
            // Elles sont dans des ensembles différents, on peut retirer le mur

            // Unir les ensembles
            self.union(cell1_index, cell2_index);

            // Supprimer le mur
            match wall_type {
                WallType::Vertical => grid.remove_vertical_wall(wall_x, wall_y),
                WallType::Horizontal => grid.remove_horizontal_wall(wall_x, wall_y),
            }

            // Retourner le changement
            return (GenerationResult::Continue, vec![WallChange {
                x: wall_x,
                y: wall_y,
                wall_type,
            }]);
        }

        // Les cellules sont déjà connectées, ne pas retirer le mur
        (GenerationResult::Continue, Vec::new())
    }

    fn is_finished(&self) -> bool {
        self.is_finished
    }

    fn get_name(&self) -> &'static str {
        "Kruskal's Algorithm"
    }

    fn get_cell_layers(&self) -> Vec<Vec<Point>> {
        // Layer 0 : La cellule courante
        if let Some(point) = self.current_point {
            vec![vec![point]]
        } else {
            vec![]
        }
    }
}
