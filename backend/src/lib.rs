mod generators;
mod maze_grid;
mod utils;

use generators::{GenerationResult, AlgorithmKind, Algorithm};
use maze_grid::MazeGrid;
use wasm_bindgen::prelude::*;
use crate::generators::GenerationAlgorithm;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, Pathfinding!");
}

use crate::utils::set_panic_hook;

// Structure pour représenter un point sur la grille
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

// Notre générateur de labyrinthe avec système de cloisons
#[wasm_bindgen]
pub struct MazeGenerator {
    grid: MazeGrid,
    active_generator: Algorithm,
}

// Méthodes WASM
#[wasm_bindgen]
impl MazeGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> MazeGenerator {
        if width == 0 || height == 0 || width > 10000 || height > 10000 {
            panic!("Invalid dimensions");
        }

        set_panic_hook();

        MazeGenerator {
            grid: MazeGrid::new(width, height),
            active_generator: Algorithm::new(AlgorithmKind::Backtracking),
        }
    }

    // Vérifier s'il y a un mur vertical
    pub fn has_vertical_wall(&self, x: usize, y: usize) -> bool {
        if self.grid.is_valid_point(x, y) {
            let index = self.grid.get_index(x, y);
            self.grid.vertical_walls[index]
        } else {
            false
        }
    }

    // Vérifier s'il y a un mur horizontal
    pub fn has_horizontal_wall(&self, x: usize, y: usize) -> bool {
        if self.grid.is_valid_point(x, y) {
            let index = self.grid.get_index(x, y);
            self.grid.horizontal_walls[index]
        } else {
            false
        }
    }

    // Nettoyer la grille (supprimer tous les murs)
    pub fn clear_grid(&mut self) {
        self.grid.clear_grid()
    }

    // Remplir la grille de murs (labyrinthe plein)
    pub fn fill_grid(&mut self) {
        self.grid.fill_grid();
    }

    // Générer un labyrinthe complet instantanément avec Backtracking
    #[wasm_bindgen]
    pub fn generate_maze(&mut self, algorithm: AlgorithmKind) {
        self.start_generation(algorithm);
        while self.generation_step() == false {}
    }

    // Méthode unifiée pour démarrer la génération avec animation
    pub fn start_generation(&mut self, algorithm: AlgorithmKind) {
        self.active_generator = Algorithm::new(algorithm);
        self.active_generator.start(&mut self.grid);
    }

    // Effectuer une étape de génération
    pub fn generation_step(&mut self) -> bool {
        let (result, _wall_changes) = self.active_generator.step(&mut self.grid);
        match result {
            GenerationResult::Continue => false,
            GenerationResult::Finished => true,
        }
    }

    // Effectuer une étape de génération avec changements de murs
    pub fn generation_step_with_changes(&mut self) -> js_sys::Object {
        let (result, wall_changes) = self.active_generator.step(&mut self.grid);
        
        let is_finished = match result {
            GenerationResult::Continue => false,
            GenerationResult::Finished => true,
        };

        let result_obj = js_sys::Object::new();
        js_sys::Reflect::set(&result_obj, &"isFinished".into(), &is_finished.into()).unwrap();
        
        let changes_array = js_sys::Array::new();
        for change in wall_changes {
            // Utiliser les structures Rust générées par wasm-bindgen
            changes_array.push(&JsValue::from(change));
        }
        js_sys::Reflect::set(&result_obj, &"changes".into(), &changes_array).unwrap();
        
        result_obj
    }

    // Obtenir les layers de cellules pour l'animation
    // Retourne un tableau de tableaux : [[layer0_cells], [layer1_cells], ...]
    pub fn get_cell_layers(&self) -> js_sys::Array {
        let layers = self.active_generator.get_cell_layers();
        let layers_array = js_sys::Array::new();

        for layer in layers {
            let layer_array = js_sys::Array::new();
            for point in layer {
                layer_array.push(&JsValue::from(point));
            }
            layers_array.push(&layer_array);
        }

        layers_array
    }

    // Redimensionner la grille
    pub fn resize_grid(&mut self, new_width: usize, new_height: usize) {
        if new_width == 0 || new_height == 0 || new_width > 10000 || new_height > 10000 {
            panic!("Invalid dimensions");
        }
        
        self.grid.resize(new_width, new_height);
    }

    // Obtenir les dimensions actuelles
    pub fn get_grid_width(&self) -> usize {
        self.grid.width
    }

    pub fn get_grid_height(&self) -> usize {
        self.grid.height
    }
}
