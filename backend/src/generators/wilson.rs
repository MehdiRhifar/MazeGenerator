use super::{GenerationAlgorithm, GenerationResult, WallChange, WallType};
use crate::maze_grid::MazeGrid;
use crate::Point;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
enum WilsonState {
    // Initialisation : choisir une nouvelle cellule de départ
    PickingStart,
    // En train de faire une marche aléatoire
    Walking,
    // Carving : ajouter le chemin au labyrinthe
    CarvingPath,
    // Terminé
    Finished,
}

pub struct WilsonGenerator {
    // État des cellules : true = dans le labyrinthe, false = hors du labyrinthe
    in_maze: Vec<bool>,

    // Chemin de la marche aléatoire courante (liste simple de points)
    current_path: Vec<Point>,

    // Liste des cellules pas encore dans le labyrinthe (pour optimisation)
    remaining_cells: Vec<Point>,

    // État de la génération
    state: WilsonState,

    // Dimensions
    width: usize,
    height: usize,
}

impl Default for WilsonGenerator {
    fn default() -> Self {
        Self {
            in_maze: Vec::new(),
            current_path: Vec::new(),
            remaining_cells: Vec::new(),
            state: WilsonState::PickingStart,
            width: 0,
            height: 0,
        }
    }
}

impl WilsonGenerator {

    fn get_valid_neighbors(&self, _grid: &MazeGrid, point: Point) -> Vec<Point> {
        let mut neighbors = Vec::new();

        // Est
        if point.x + 1 < self.width {
            neighbors.push(Point { x: point.x + 1, y: point.y });
        }
        // Ouest
        if point.x > 0 {
            neighbors.push(Point { x: point.x - 1, y: point.y });
        }
        // Sud
        if point.y + 1 < self.height {
            neighbors.push(Point { x: point.x, y: point.y + 1 });
        }
        // Nord
        if point.y > 0 {
            neighbors.push(Point { x: point.x, y: point.y - 1 });
        }

        neighbors
    }


    fn step_picking_start(&mut self, _grid: &MazeGrid) -> (GenerationResult, Vec<WallChange>) {
        // Si toutes les cellules sont dans le labyrinthe, terminé
        if self.remaining_cells.is_empty() {
            self.state = WilsonState::Finished;
            self.current_path.clear();
            return (GenerationResult::Finished, Vec::new());
        }

        // Choisir une cellule aléatoire parmi celles pas encore dans le labyrinthe
        let mut rng = rand::rng();
        let start_index = rng.random_range(0..self.remaining_cells.len());
        let start_point = self.remaining_cells[start_index];

        // Démarrer un nouveau chemin avec ce point
        self.current_path = vec![start_point];

        // Passer à l'état marche
        self.state = WilsonState::Walking;

        (GenerationResult::Continue, Vec::new())
    }

    fn step_walking(&mut self, grid: &MazeGrid) -> (GenerationResult, Vec<WallChange>) {
        // La dernière cellule du chemin est la position actuelle
        let current = *self.current_path.last().unwrap();
        let current_index = grid.get_index(current.x, current.y);

        // Vérifier si on a atteint le labyrinthe
        if self.in_maze[current_index] {
            // On a atteint le labyrinthe, passer à l'état "carving"
            self.state = WilsonState::CarvingPath;
            return (GenerationResult::Continue, Vec::new());
        }

        // Choisir une direction aléatoire
        let neighbors = self.get_valid_neighbors(grid, current);
        let mut rng = rand::rng();
        let next_point = neighbors[rng.random_range(0..neighbors.len())];

        // Vérifier si on crée une boucle (next_point est déjà dans le chemin)
        if let Some(loop_index) = self.current_path.iter().position(|&p| p == next_point) {
            // Boucle détectée : garder seulement le chemin jusqu'à loop_index (inclus)
            self.current_path.truncate(loop_index + 1);
        } else {
            // Pas de boucle : ajouter le nouveau point au chemin
            self.current_path.push(next_point);
        }

        (GenerationResult::Continue, Vec::new())
    }

    fn step_carving_path(&mut self, grid: &mut MazeGrid) -> (GenerationResult, Vec<WallChange>) {
        // Le chemin ne devrait pas être vide ici
        if self.current_path.is_empty() {
            self.state = WilsonState::PickingStart;
            return (GenerationResult::Continue, Vec::new());
        }

        // Prendre la dernière cellule du chemin (fin de la pile)
        let current = *self.current_path.last().unwrap();
        let current_index = grid.get_index(current.x, current.y);

        // Ajouter la cellule courante au labyrinthe
        self.in_maze[current_index] = true;

        // Retirer la cellule de remaining_cells
        if let Some(pos) = self.remaining_cells.iter().position(|p| *p == current) {
            self.remaining_cells.swap_remove(pos);
        }

        // S'il n'y a qu'une cellule dans le chemin, on a fini
        if self.current_path.len() == 1 {
            self.current_path.clear();
            self.state = WilsonState::PickingStart;
            return (GenerationResult::Continue, Vec::new());
        }

        // Sinon, supprimer le mur entre current et la cellule précédente (avant-dernière)
        let next = self.current_path[self.current_path.len() - 2];

        // Déterminer quel mur supprimer
        let wall_change = if next.x > current.x {
            // Mur vertical à droite de current
            grid.remove_vertical_wall(current.x, current.y);
            WallChange { x: current.x, y: current.y, wall_type: WallType::Vertical }
        } else if next.x < current.x {
            // Mur vertical à gauche de current (= à droite de next)
            grid.remove_vertical_wall(next.x, next.y);
            WallChange { x: next.x, y: next.y, wall_type: WallType::Vertical }
        } else if next.y > current.y {
            // Mur horizontal en bas de current
            grid.remove_horizontal_wall(current.x, current.y);
            WallChange { x: current.x, y: current.y, wall_type: WallType::Horizontal }
        } else {
            // Mur horizontal en haut de current (= en bas de next)
            grid.remove_horizontal_wall(next.x, next.y);
            WallChange { x: next.x, y: next.y, wall_type: WallType::Horizontal }
        };

        // Retirer la dernière cellule du chemin
        self.current_path.pop();

        (GenerationResult::Continue, vec![wall_change])
    }
}

impl GenerationAlgorithm for WilsonGenerator {
    fn start(&mut self, grid: &mut MazeGrid) {
        self.width = grid.width;
        self.height = grid.height;
        let total_cells = self.width * self.height;

        // Initialiser les états
        self.in_maze = vec![false; total_cells];
        self.current_path = Vec::new();

        // Remplir la grille de murs
        grid.fill_grid();

        // Créer la liste de toutes les cellules
        self.remaining_cells = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                self.remaining_cells.push(Point { x, y });
            }
        }

        // Choisir une cellule aléatoire comme point de départ du labyrinthe
        let mut rng = rand::rng();
        let start_index = rng.random_range(0..self.remaining_cells.len());
        let start_point = self.remaining_cells.swap_remove(start_index);

        let maze_start_index = grid.get_index(start_point.x, start_point.y);
        self.in_maze[maze_start_index] = true;

        // Commencer avec l'état "choisir une nouvelle cellule"
        self.state = WilsonState::PickingStart;
    }

    fn step(&mut self, grid: &mut MazeGrid) -> (GenerationResult, Vec<WallChange>) {
        match self.state {
            WilsonState::PickingStart => self.step_picking_start(grid),
            WilsonState::Walking => self.step_walking(grid),
            WilsonState::CarvingPath => self.step_carving_path(grid),
            WilsonState::Finished => (GenerationResult::Finished, Vec::new()),
        }
    }

    fn is_finished(&self) -> bool {
        self.state == WilsonState::Finished
    }

    fn get_name(&self) -> &'static str {
        "Wilson's Algorithm"
    }

    fn get_cell_layers(&self) -> Vec<Vec<Point>> {
        let mut layers = Vec::new();

        // Layer 0 : Cellules dans le labyrinthe (vert - le "but")
        let mut maze_cells = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                if self.in_maze[index] {
                    maze_cells.push(Point { x, y });
                }
            }
        }
        layers.push(maze_cells);

        // Layer 1 : Le chemin de la marche aléatoire (bleu)
        layers.push(self.current_path.clone());

        layers
    }
}
