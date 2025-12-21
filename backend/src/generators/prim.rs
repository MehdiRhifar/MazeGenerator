use super::{GenerationAlgorithm, GenerationResult, WallChange, WallType};
use crate::maze_grid::MazeGrid;
use crate::Point;
use rand::Rng;

// Structure pour représenter un mur de frontière
struct Wall {
    x: usize,
    y: usize,
    wall_type: WallType,
    neighbor_x: usize,
    neighbor_y: usize,
}

#[derive(Default)]
pub struct PrimGenerator {
    // État des cellules : true = dans le labyrinthe, false = hors du labyrinthe
    in_maze: Vec<bool>,

    // Liste des murs de frontière (murs entre cellules "in" et "out")
    frontier_walls: Vec<Wall>,

    // Position courante pour l'animation (dernière cellule ajoutée)
    current_point: Option<Point>,

    // État de fin
    is_finished: bool,

    // Dimensions
    width: usize,
    height: usize,
}

impl PrimGenerator {

    fn add_walls_to_frontier(&mut self, grid: &MazeGrid, point: Point) {
        // Vérifier droite (mur vertical)
        if point.x < self.width - 1 {
            let neighbor_x = point.x + 1;
            let neighbor_index = grid.get_index(neighbor_x, point.y);
            if !self.in_maze[neighbor_index] {
                self.frontier_walls.push(Wall {
                    x: point.x,
                    y: point.y,
                    wall_type: WallType::Vertical,
                    neighbor_x,
                    neighbor_y: point.y,
                });
            }
        }

        // Vérifier bas (mur horizontal)
        if point.y < self.height - 1 {
            let neighbor_y = point.y + 1;
            let neighbor_index = grid.get_index(point.x, neighbor_y);
            if !self.in_maze[neighbor_index] {
                self.frontier_walls.push(Wall {
                    x: point.x,
                    y: point.y,
                    wall_type: WallType::Horizontal,
                    neighbor_x: point.x,
                    neighbor_y,
                });
            }
        }

        // Vérifier gauche (mur vertical défini par le voisin)
        if point.x > 0 {
            let neighbor_x = point.x - 1;
            let neighbor_index = grid.get_index(neighbor_x, point.y);
            if !self.in_maze[neighbor_index] {
                self.frontier_walls.push(Wall {
                    x: neighbor_x,
                    y: point.y,
                    wall_type: WallType::Vertical,
                    neighbor_x,
                    neighbor_y: point.y,
                });
            }
        }

        // Vérifier haut (mur horizontal défini par le voisin)
        if point.y > 0 {
            let neighbor_y = point.y - 1;
            let neighbor_index = grid.get_index(point.x, neighbor_y);
            if !self.in_maze[neighbor_index] {
                self.frontier_walls.push(Wall {
                    x: point.x,
                    y: neighbor_y,
                    wall_type: WallType::Horizontal,
                    neighbor_x: point.x,
                    neighbor_y,
                });
            }
        }
    }
}

impl GenerationAlgorithm for PrimGenerator {
    fn start(&mut self, grid: &mut MazeGrid) {
        self.width = grid.width;
        self.height = grid.height;
        self.in_maze = vec![false; self.width * self.height];
        self.frontier_walls = Vec::new();
        self.is_finished = false;

        // Remplir la grille de murs
        grid.fill_grid();

        // Choisir une cellule de départ aléatoire
        let mut rng = rand::rng();
        let start_x = rng.random_range(0..self.width);
        let start_y = rng.random_range(0..self.height);
        let start_point = Point { x: start_x, y: start_y };

        // Marquer cette cellule comme dans le labyrinthe
        let start_index = grid.get_index(start_x, start_y);
        self.in_maze[start_index] = true;
        self.current_point = Some(start_point);

        // Ajouter tous les murs de cette cellule à la frontière
        self.add_walls_to_frontier(grid, start_point);
    }

    fn step(&mut self, grid: &mut MazeGrid) -> (GenerationResult, Vec<WallChange>) {
        // Si la frontière est vide, terminé
        if self.frontier_walls.is_empty() {
            self.is_finished = true;
            self.current_point = None;
            return (GenerationResult::Finished, Vec::new());
        }

        // Choisir un mur aléatoire de la frontière
        let mut rng = rand::rng();
        let wall_index = rng.random_range(0..self.frontier_walls.len());
        let wall = self.frontier_walls.swap_remove(wall_index);

        // Vérifier si le voisin est toujours hors du labyrinthe
        let neighbor_index = grid.get_index(wall.neighbor_x, wall.neighbor_y);

        if !self.in_maze[neighbor_index] {
            // Le voisin est hors du labyrinthe, on peut le connecter

            // Supprimer le mur
            match wall.wall_type {
                WallType::Vertical => grid.remove_vertical_wall(wall.x, wall.y),
                WallType::Horizontal => grid.remove_horizontal_wall(wall.x, wall.y),
            }

            // Marquer le voisin comme dans le labyrinthe
            self.in_maze[neighbor_index] = true;

            // Mettre à jour la position courante
            self.current_point = Some(Point {
                x: wall.neighbor_x,
                y: wall.neighbor_y
            });

            // Ajouter les murs du voisin à la frontière
            self.add_walls_to_frontier(grid, Point {
                x: wall.neighbor_x,
                y: wall.neighbor_y,
            });

            // Retourner le changement de mur
            return (GenerationResult::Continue, vec![WallChange {
                x: wall.x,
                y: wall.y,
                wall_type: wall.wall_type,
            }]);
        }

        // Le voisin est déjà dans le labyrinthe, continuer sans changement
        (GenerationResult::Continue, Vec::new())
    }

    fn is_finished(&self) -> bool {
        self.is_finished
    }

    fn get_name(&self) -> &'static str {
        "Randomized Prim"
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
