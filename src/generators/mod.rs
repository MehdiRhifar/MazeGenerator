use crate::Point;
use crate::maze_grid::MazeGrid;
use wasm_bindgen::prelude::*;

pub mod backtracking;
pub use backtracking::BacktrackingGenerator;

#[derive(Debug)]
pub enum GenerationResult {
    Continue,
    Finished,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WallChange {
    pub x: usize,
    pub y: usize,
    pub wall_type: WallType,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum WallType {
    Vertical = 0,
    Horizontal = 1,
}

// Note: Vec<WallChange> n'est pas supporté par wasm-bindgen
// On va gérer cela différemment dans lib.rs

// Enum pour identifier les types de générateurs
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneratorType {
    Backtracking,
    // Futurs algorithmes :
    // Prim,
    // Kruskal,
    // Wilson,
}


// Enum contenant les générateurs avec dispatch intégré
pub enum Algorithm {
    Backtracking(BacktrackingGenerator),
    // Futurs algorithmes :
    // Prim(PrimGenerator),
    // Kruskal(KruskalGenerator),
    // Wilson(WilsonGenerator),
}

impl Algorithm {
    
    /// Créer un nouvel algorithme selon le type demandé
    pub(crate) fn new(generator_type: GeneratorType) -> Self {
        match generator_type {
            GeneratorType::Backtracking => Self::Backtracking(BacktrackingGenerator::new()),
            // GeneratorType::Prim => Self::Prim(PrimGenerator::new()),
        }
    }
}

impl GenerationAlgorithm for Algorithm {
    /// Démarrer la génération
    fn start(&mut self, grid: &mut MazeGrid) {
        match self {
            Self::Backtracking(gen) => gen.start(grid),
            // Self::Prim(gen) => gen.start(grid),
        }
    }

    /// Effectuer une étape de génération
    fn step(&mut self, grid: &mut MazeGrid) -> (GenerationResult, Vec<WallChange>) {
        match self {
            Self::Backtracking(gen) => gen.step(grid),
            // Self::Prim(gen) => gen.step(grid),
        }
    }
    
    /// Vérifier si la génération est terminée
    fn is_finished(&self) -> bool {
        match self {
            Self::Backtracking(gen) => gen.is_finished(),
            // Self::Prim(gen) => gen.is_finished(),
        }
    }
    
    /// Obtenir la position actuelle (pour l'animation)
    fn get_current_position(&self) -> Option<Point> {
        match self {
            Self::Backtracking(gen) => gen.get_current_position(),
            // Self::Prim(gen) => gen.get_current_position(),
        }
    }
    
    /// Obtenir le nom de l'algorithme
    fn get_name(&self) -> &'static str {
        match self {
            Self::Backtracking(gen) => gen.get_name(),
            // Self::Prim(gen) => gen.get_name(),
        }
    }
}

pub trait GenerationAlgorithm {
    fn start(&mut self, grid: &mut MazeGrid);
    fn step(&mut self, grid: &mut MazeGrid) -> (GenerationResult, Vec<WallChange>);
    fn is_finished(&self) -> bool;
    fn get_current_position(&self) -> Option<Point>;
    fn get_name(&self) -> &'static str;
}