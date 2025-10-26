use crate::Point;

/// Structure contenant les données de grille du labyrinthe
/// Sépare les données de la logique de génération
/// 
/// Convention des murs :
/// - vertical_walls[i] : mur vertical à droite de la cellule i
/// - horizontal_walls[i] : mur horizontal en bas de la cellule i
/// - Les bordures du labyrinthe sont toujours considérées comme des murs
/// - Une cellule (x,y) a l'index : y * width + x
pub struct MazeGrid {
    pub vertical_walls: Vec<bool>,
    pub horizontal_walls: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

impl MazeGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let total_cells = width * height;
        Self {
            vertical_walls: vec![false; total_cells],
            horizontal_walls: vec![false; total_cells],
            width,
            height,
        }
    }
}

impl MazeGrid {
    pub fn is_valid_point(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn remove_vertical_wall(&mut self, x: usize, y: usize) {
        if self.is_valid_point(x, y) {
            let index = self.get_index(x, y);
            self.vertical_walls[index] = false;
        }
    }

    pub fn remove_horizontal_wall(&mut self, x: usize, y: usize) {
        if self.is_valid_point(x, y) {
            let index = self.get_index(x, y);
            self.horizontal_walls[index] = false;
        }
    }

    pub fn remove_wall_between(&mut self, from: Point, to: Point) {
        if from.x == to.x {
            // Mouvement vertical
            let y_min = if from.y < to.y { from.y } else { to.y };
            self.remove_horizontal_wall(from.x, y_min);
        } else if from.y == to.y {
            // Mouvement horizontal
            let x_min = if from.x < to.x { from.x } else { to.x };
            self.remove_vertical_wall(x_min, from.y);
        }
    }

    pub fn fill_grid(&mut self) {
        self.vertical_walls.fill(true);
        self.horizontal_walls.fill(true);
    }

    pub fn clear_grid(&mut self) {
        self.vertical_walls.fill(false);
        self.horizontal_walls.fill(false);
    }

    /// Redimensionne la grille en conservant les murs existants quand c'est possible
    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        if new_width == self.width && new_height == self.height {
            return; // Pas de changement
        }

        let new_total_cells = new_width * new_height;
        let mut new_vertical_walls = vec![false; new_total_cells];
        let mut new_horizontal_walls = vec![false; new_total_cells];

        // Copier les murs existants dans les limites communes
        let min_width = self.width.min(new_width);
        let min_height = self.height.min(new_height);

        for y in 0..min_height {
            for x in 0..min_width {
                let old_index = self.get_index(x, y);
                let new_index = y * new_width + x;

                // Copier les murs si ils sont dans les limites
                if x < min_width {
                    new_vertical_walls[new_index] = self.vertical_walls[old_index];
                }
                if y < min_height {
                    new_horizontal_walls[new_index] = self.horizontal_walls[old_index];
                }
            }
        }

        // Mettre à jour la grille
        self.vertical_walls = new_vertical_walls;
        self.horizontal_walls = new_horizontal_walls;
        self.width = new_width;
        self.height = new_height;
    }
}