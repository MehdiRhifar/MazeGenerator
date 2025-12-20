use super::{GenerationAlgorithm, GenerationResult, WallChange, WallType};
use crate::maze_grid::MazeGrid;
use crate::Point;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

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

    // Chemin de la marche aléatoire courante
    // path[i] = direction pour aller de la cellule i vers la suivante dans le chemin
    // None si la cellule n'est pas dans le chemin courant
    path: Vec<Option<Direction>>,

    // Cellule de départ de la marche courante
    walk_start: Option<Point>,

    // Position courante dans la marche
    current_position: Option<Point>,

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
            path: Vec::new(),
            walk_start: None,
            current_position: None,
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

    fn erase_loop(&mut self, grid: &MazeGrid, loop_start: Point) {
        // Effacer le chemin depuis loop_start jusqu'à ce qu'on revienne à loop_start
        let mut current = loop_start;

        loop {
            let current_index = grid.get_index(current.x, current.y);

            // Obtenir la direction suivante avant de l'effacer
            let direction = self.path[current_index];

            // Effacer cette étape du chemin
            self.path[current_index] = None;

            if let Some(dir) = direction {
                // Avancer dans cette direction
                current = match dir {
                    Direction::North => Point { x: current.x, y: current.y - 1 },
                    Direction::South => Point { x: current.x, y: current.y + 1 },
                    Direction::East => Point { x: current.x + 1, y: current.y },
                    Direction::West => Point { x: current.x - 1, y: current.y },
                };

                // Si on revient au point de départ de la boucle, arrêter
                if current == loop_start {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn step_picking_start(&mut self, _grid: &MazeGrid) -> (GenerationResult, Vec<WallChange>) {
        // Si toutes les cellules sont dans le labyrinthe, terminé
        if self.remaining_cells.is_empty() {
            self.state = WilsonState::Finished;
            self.current_position = None;
            return (GenerationResult::Finished, Vec::new());
        }

        // Choisir une cellule aléatoire parmi celles pas encore dans le labyrinthe
        let mut rng = rand::rng();
        let start_index = rng.random_range(0..self.remaining_cells.len());
        let start_point = self.remaining_cells[start_index];

        self.walk_start = Some(start_point);
        self.current_position = Some(start_point);

        // Réinitialiser le chemin
        self.path.fill(None);

        // Passer à l'état marche
        self.state = WilsonState::Walking;

        (GenerationResult::Continue, Vec::new())
    }

    fn step_walking(&mut self, grid: &MazeGrid) -> (GenerationResult, Vec<WallChange>) {
        let current = self.current_position.unwrap();
        let current_index = grid.get_index(current.x, current.y);

        // Vérifier si on a atteint le labyrinthe
        if self.in_maze[current_index] {
            // On a atteint le labyrinthe, passer à l'état "carving"
            self.state = WilsonState::CarvingPath;
            self.current_position = self.walk_start;
            return (GenerationResult::Continue, Vec::new());
        }

        // Choisir une direction aléatoire
        let neighbors = self.get_valid_neighbors(grid, current);
        let mut rng = rand::rng();
        let next_point = neighbors[rng.random_range(0..neighbors.len())];

        // Déterminer la direction
        let direction = if next_point.x > current.x {
            Direction::East
        } else if next_point.x < current.x {
            Direction::West
        } else if next_point.y > current.y {
            Direction::South
        } else {
            Direction::North
        };

        // Vérifier si on crée une boucle
        let next_index = grid.get_index(next_point.x, next_point.y);
        if self.path[next_index].is_some() {
            // On crée une boucle, effacer depuis next_point jusqu'à current
            self.erase_loop(grid, next_point);
        }

        // Ajouter la direction au chemin
        self.path[current_index] = Some(direction);

        // Avancer
        self.current_position = Some(next_point);

        (GenerationResult::Continue, Vec::new())
    }

    fn step_carving_path(&mut self, grid: &mut MazeGrid) -> (GenerationResult, Vec<WallChange>) {
        let current = self.current_position.unwrap();
        let current_index = grid.get_index(current.x, current.y);

        // Si on a atteint le labyrinthe, terminé avec ce chemin
        if self.in_maze[current_index] {
            // Retourner à l'état "choisir une nouvelle cellule"
            self.state = WilsonState::PickingStart;
            return (GenerationResult::Continue, Vec::new());
        }

        // Ajouter la cellule courante au labyrinthe
        self.in_maze[current_index] = true;

        // Retirer la cellule de remaining_cells
        if let Some(pos) = self.remaining_cells.iter().position(|p| *p == current) {
            self.remaining_cells.swap_remove(pos);
        }

        // Obtenir la direction suivante - si None, on a fini ce chemin
        let Some(direction) = self.path[current_index] else {
            // Pas de direction, retourner à picking start
            self.state = WilsonState::PickingStart;
            return (GenerationResult::Continue, Vec::new());
        };

        // Déterminer le point suivant
        let next_point = match direction {
            Direction::North => Point { x: current.x, y: current.y - 1 },
            Direction::South => Point { x: current.x, y: current.y + 1 },
            Direction::East => Point { x: current.x + 1, y: current.y },
            Direction::West => Point { x: current.x - 1, y: current.y },
        };

        // Supprimer le mur entre current et next
        let wall_change = match direction {
            Direction::East => {
                grid.remove_vertical_wall(current.x, current.y);
                WallChange {
                    x: current.x,
                    y: current.y,
                    wall_type: WallType::Vertical,
                }
            }
            Direction::West => {
                grid.remove_vertical_wall(next_point.x, next_point.y);
                WallChange {
                    x: next_point.x,
                    y: next_point.y,
                    wall_type: WallType::Vertical,
                }
            }
            Direction::South => {
                grid.remove_horizontal_wall(current.x, current.y);
                WallChange {
                    x: current.x,
                    y: current.y,
                    wall_type: WallType::Horizontal,
                }
            }
            Direction::North => {
                grid.remove_horizontal_wall(next_point.x, next_point.y);
                WallChange {
                    x: next_point.x,
                    y: next_point.y,
                    wall_type: WallType::Horizontal,
                }
            }
        };

        // Avancer
        self.current_position = Some(next_point);

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
        self.path = vec![None; total_cells];

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
        self.walk_start = None;
        self.current_position = Some(start_point);
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

    fn get_current_position(&self) -> Option<Point> {
        self.current_position
    }

    fn get_name(&self) -> &'static str {
        "Wilson's Algorithm"
    }
}
