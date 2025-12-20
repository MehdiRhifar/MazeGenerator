use super::{GenerationAlgorithm, GenerationResult, WallChange, WallType};
use crate::maze_grid::MazeGrid;
use crate::Point;
use rand::Rng;

// Structure pour représenter une chambre à diviser
struct Chamber {
    x_min: usize,
    y_min: usize,
    x_max: usize,  // Exclusif
    y_max: usize,  // Exclusif
}

impl Chamber {
    fn width(&self) -> usize {
        self.x_max - self.x_min
    }

    fn height(&self) -> usize {
        self.y_max - self.y_min
    }

    fn can_divide_horizontally(&self) -> bool {
        self.height() >= 2
    }

    fn can_divide_vertically(&self) -> bool {
        self.width() >= 2
    }
}

#[derive(Default)]
pub struct RecursiveDivisionGenerator {
    // Pile des chambres à diviser
    chambers: Vec<Chamber>,

    // Position courante pour l'animation (dernier mur ajouté)
    current_point: Option<Point>,

    // État de fin
    is_finished: bool,

    // Dimensions
    width: usize,
    height: usize,
}

impl RecursiveDivisionGenerator {

    fn divide_chamber_horizontally(
        &mut self,
        grid: &mut MazeGrid,
        chamber: Chamber,
        rng: &mut impl rand::RngCore,
    ) -> (GenerationResult, Vec<WallChange>) {
        // Choisir une ligne pour le mur horizontal
        let wall_y = chamber.y_min + rng.random_range(0..(chamber.height() - 1));

        // Choisir une position pour le passage
        let passage_x = chamber.x_min + rng.random_range(0..chamber.width());

        // Ajouter le mur horizontal, sauf au passage
        let mut wall_changes = Vec::new();

        for x in chamber.x_min..chamber.x_max {
            if x != passage_x {
                grid.add_horizontal_wall(x, wall_y);
                wall_changes.push(WallChange {
                    x,
                    y: wall_y,
                    wall_type: WallType::Horizontal,
                });
            }
        }

        // Mettre à jour la position courante
        self.current_point = Some(Point { x: passage_x, y: wall_y });

        // Ajouter les deux sous-chambres à la pile
        self.chambers.push(Chamber {
            x_min: chamber.x_min,
            y_min: chamber.y_min,
            x_max: chamber.x_max,
            y_max: wall_y + 1,
        });

        self.chambers.push(Chamber {
            x_min: chamber.x_min,
            y_min: wall_y + 1,
            x_max: chamber.x_max,
            y_max: chamber.y_max,
        });

        (GenerationResult::Continue, wall_changes)
    }

    fn divide_chamber_vertically(
        &mut self,
        grid: &mut MazeGrid,
        chamber: Chamber,
        rng: &mut impl rand::RngCore,
    ) -> (GenerationResult, Vec<WallChange>) {
        // Choisir une colonne pour le mur vertical
        let wall_x = chamber.x_min + rng.random_range(0..(chamber.width() - 1));

        // Choisir une position pour le passage
        let passage_y = chamber.y_min + rng.random_range(0..chamber.height());

        // Ajouter le mur vertical, sauf au passage
        let mut wall_changes = Vec::new();

        for y in chamber.y_min..chamber.y_max {
            if y != passage_y {
                grid.add_vertical_wall(wall_x, y);
                wall_changes.push(WallChange {
                    x: wall_x,
                    y,
                    wall_type: WallType::Vertical,
                });
            }
        }

        // Mettre à jour la position courante
        self.current_point = Some(Point { x: wall_x, y: passage_y });

        // Ajouter les deux sous-chambres à la pile
        self.chambers.push(Chamber {
            x_min: chamber.x_min,
            y_min: chamber.y_min,
            x_max: wall_x + 1,
            y_max: chamber.y_max,
        });

        self.chambers.push(Chamber {
            x_min: wall_x + 1,
            y_min: chamber.y_min,
            x_max: chamber.x_max,
            y_max: chamber.y_max,
        });

        (GenerationResult::Continue, wall_changes)
    }
}

impl GenerationAlgorithm for RecursiveDivisionGenerator {
    fn start(&mut self, grid: &mut MazeGrid) {
        self.width = grid.width;
        self.height = grid.height;

        // IMPORTANT : Commencer avec une grille VIDE (pas de murs)
        grid.clear_grid();

        // Ajouter la chambre initiale (toute la grille)
        self.chambers = vec![Chamber {
            x_min: 0,
            y_min: 0,
            x_max: self.width,
            y_max: self.height,
        }];

        self.is_finished = false;
        self.current_point = Some(Point { x: 0, y: 0 });
    }

    fn step(&mut self, grid: &mut MazeGrid) -> (GenerationResult, Vec<WallChange>) {
        // Si la pile est vide, terminé
        if self.chambers.is_empty() {
            self.is_finished = true;
            self.current_point = None;
            return (GenerationResult::Finished, Vec::new());
        }

        // Prendre une chambre de la pile
        let chamber = self.chambers.pop().unwrap();

        // Vérifier si on peut diviser cette chambre
        let can_horizontal = chamber.can_divide_horizontally();
        let can_vertical = chamber.can_divide_vertically();

        if !can_horizontal && !can_vertical {
            // Chambre trop petite, ne rien faire
            return (GenerationResult::Continue, Vec::new());
        }

        // Choisir l'orientation
        let mut rng = rand::rng();
        let divide_horizontally = if can_horizontal && can_vertical {
            // Les deux sont possibles, choisir avec biais selon les proportions
            if chamber.width() > chamber.height() {
                rng.random_range(0..4) == 0  // 25% de chance de horizontal si plus large
            } else if chamber.height() > chamber.width() {
                rng.random_range(0..4) != 0  // 75% de chance de horizontal si plus haut
            } else {
                rng.random_range(0..2) == 0  // 50/50 si carré
            }
        } else {
            can_horizontal
        };

        if divide_horizontally {
            self.divide_chamber_horizontally(grid, chamber, &mut rng)
        } else {
            self.divide_chamber_vertically(grid, chamber, &mut rng)
        }
    }

    fn is_finished(&self) -> bool {
        self.is_finished
    }

    fn get_current_position(&self) -> Option<Point> {
        self.current_point
    }

    fn get_name(&self) -> &'static str {
        "Recursive Division"
    }
}
