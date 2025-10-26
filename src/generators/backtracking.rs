use super::{GenerationAlgorithm, GenerationResult, WallChange, WallType};
use crate::maze_grid::MazeGrid;
use crate::Point;
use rand::Rng;

pub struct BacktrackingGenerator {
    visited: Vec<bool>,
    stack: Vec<Point>,
    current_point: Option<Point>,
    is_finished: bool,
    width: usize,
    height: usize,
}

impl BacktrackingGenerator {
    pub fn new() -> Self {
        Self {
            visited: Vec::new(),
            stack: Vec::new(),
            current_point: None,
            is_finished: false,
            width: 0,
            height: 0,
        }
    }

    fn get_random_unvisited_neighbors(&self, grid: &MazeGrid, point: Point) -> Option<Point> {
        let candidates = [
            (point.x, point.y + 1),
            (point.x, point.y - 1),
            (point.x + 1, point.y),
            (point.x - 1, point.y),
        ];

        let mut rng = rand::rng();
        let start = rng.random_range(0..4);

        for i in 0..4 {
            let (dx, dy) = candidates[(start + i) % 4];
            if grid.is_valid_point(dx, dy) {
                let index = grid.get_index(dx, dy);
                if !self.visited[index] {
                    return Some(Point { x: dx, y: dy });
                }
            }
        }
        None
    }

    fn remove_wall_between_with_tracking(&self, grid: &mut MazeGrid, from: Point, to: Point) -> Vec<WallChange> {
        let mut changes = Vec::new();
        
        if from.x == to.x {
            // Mouvement vertical
            let y_min = if from.y < to.y { from.y } else { to.y };
            grid.remove_horizontal_wall(from.x, y_min);
            changes.push(WallChange {
                x: from.x,
                y: y_min,
                wall_type: WallType::Horizontal,
            });
        } else if from.y == to.y {
            // Mouvement horizontal
            let x_min = if from.x < to.x { from.x } else { to.x };
            grid.remove_vertical_wall(x_min, from.y);
            changes.push(WallChange {
                x: x_min,
                y: from.y,
                wall_type: WallType::Vertical,
            });
        }
        
        changes
    }
}

impl GenerationAlgorithm for BacktrackingGenerator {
    fn start(&mut self, grid: &mut MazeGrid) {
        self.width = grid.width;
        self.height = grid.height;
        self.visited = vec![false; self.width * self.height];
        self.stack = Vec::new();
        self.is_finished = false;

        // Commencer depuis le coin supérieur gauche
        let start_point = Point { x: 0, y: 0 };
        self.stack.push(start_point);
        self.current_point = Some(start_point);
        grid.fill_grid();
    }

    fn step(&mut self, grid: &mut MazeGrid) -> (GenerationResult, Vec<WallChange>) {
        // Si pas de point courant, on a fini
        let Some(current) = self.stack.last().copied() else {
            self.is_finished = true;
            self.current_point = None;
            return (GenerationResult::Finished, Vec::new());
        };

        self.current_point = Some(current);

        // Marquer le point courant comme visité
        let current_index = grid.get_index(current.x, current.y);
        self.visited[current_index] = true;

        // Chercher un voisin non visité
        if let Some(next) = self.get_random_unvisited_neighbors(grid, current) {
            // On a trouvé un voisin, supprimer le mur et avancer
            let wall_changes = self.remove_wall_between_with_tracking(grid, current, next);
            self.stack.push(next);
            (GenerationResult::Continue, wall_changes)
        } else {
            // Pas de voisin non visité, revenir en arrière
            self.stack.pop();
            (GenerationResult::Continue, Vec::new())
        }
    }

    fn is_finished(&self) -> bool {
        self.is_finished
    }

    fn get_current_position(&self) -> Option<Point> {
        self.current_point
    }

    fn get_name(&self) -> &'static str {
        "Recursive Backtracking"
    }
}
