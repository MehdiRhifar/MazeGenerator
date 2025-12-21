use crate::Point;
use crate::maze_grid::MazeGrid;
use wasm_bindgen::prelude::*;

pub mod backtracking;
pub mod prim;
pub mod kruskal;
pub mod wilson;
pub mod recursive_division;

pub use backtracking::BacktrackingGenerator;
pub use prim::PrimGenerator;
pub use kruskal::KruskalGenerator;
pub use wilson::WilsonGenerator;
pub use recursive_division::RecursiveDivisionGenerator;

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
pub enum AlgorithmKind {
    Backtracking,
    Prim,
    Kruskal,
    Wilson,
    RecursiveDivision,
}


// Possibiliter d'utiliser enum_dispatch qui fait le dispatch automatiquement, mais on garde explicite pour l'instant
// Enum contenant les générateurs avec dispatch intégré
pub enum Algorithm {
    Backtracking(BacktrackingGenerator),
    Prim(PrimGenerator),
    Kruskal(KruskalGenerator),
    Wilson(WilsonGenerator),
    RecursiveDivision(RecursiveDivisionGenerator),
}

impl Algorithm {
    
    /// Créer un nouvel algorithme selon le type demandé
    pub(crate) fn new(generator_type: AlgorithmKind) -> Self {
        match generator_type {
            AlgorithmKind::Backtracking => Self::Backtracking(BacktrackingGenerator::default()),
            AlgorithmKind::Prim => Self::Prim(PrimGenerator::default()),
            AlgorithmKind::Kruskal => Self::Kruskal(KruskalGenerator::default()),
            AlgorithmKind::Wilson => Self::Wilson(WilsonGenerator::default()),
            AlgorithmKind::RecursiveDivision => Self::RecursiveDivision(RecursiveDivisionGenerator::default()),
        }
    }
}

impl GenerationAlgorithm for Algorithm {
    /// Démarrer la génération
    fn start(&mut self, grid: &mut MazeGrid) {
        match self {
            Self::Backtracking(generator) => generator.start(grid),
            Self::Prim(generator) => generator.start(grid),
            Self::Kruskal(generator) => generator.start(grid),
            Self::Wilson(generator) => generator.start(grid),
            Self::RecursiveDivision(generator) => generator.start(grid),
        }
    }

    /// Effectuer une étape de génération
    fn step(&mut self, grid: &mut MazeGrid) -> (GenerationResult, Vec<WallChange>) {
        match self {
            Self::Backtracking(generator) => generator.step(grid),
            Self::Prim(generator) => generator.step(grid),
            Self::Kruskal(generator) => generator.step(grid),
            Self::Wilson(generator) => generator.step(grid),
            Self::RecursiveDivision(generator) => generator.step(grid),
        }
    }
    
    /// Vérifier si la génération est terminée
    fn is_finished(&self) -> bool {
        match self {
            Self::Backtracking(generator) => generator.is_finished(),
            Self::Prim(generator) => generator.is_finished(),
            Self::Kruskal(generator) => generator.is_finished(),
            Self::Wilson(generator) => generator.is_finished(),
            Self::RecursiveDivision(generator) => generator.is_finished(),
        }
    }
    
    /// Obtenir le nom de l'algorithme
    fn get_name(&self) -> &'static str {
        match self {
            Self::Backtracking(generator) => generator.get_name(),
            Self::Prim(generator) => generator.get_name(),
            Self::Kruskal(generator) => generator.get_name(),
            Self::Wilson(generator) => generator.get_name(),
            Self::RecursiveDivision(generator) => generator.get_name(),
        }
    }

    /// Obtenir les layers de cellules à afficher
    fn get_cell_layers(&self) -> Vec<Vec<Point>> {
        match self {
            Self::Backtracking(generator) => generator.get_cell_layers(),
            Self::Prim(generator) => generator.get_cell_layers(),
            Self::Kruskal(generator) => generator.get_cell_layers(),
            Self::Wilson(generator) => generator.get_cell_layers(),
            Self::RecursiveDivision(generator) => generator.get_cell_layers(),
        }
    }
}

pub trait GenerationAlgorithm {
    fn start(&mut self, grid: &mut MazeGrid);
    fn step(&mut self, grid: &mut MazeGrid) -> (GenerationResult, Vec<WallChange>);
    fn is_finished(&self) -> bool;
    fn get_name(&self) -> &'static str;

    /// Retourner les layers de cellules à afficher avec des couleurs différentes
    /// - Layer 0 : Cellules principales (ex: chemin actuel)
    /// - Layer 1 : Cellules secondaires (ex: dans le labyrinthe)
    /// - Layer 2+ : Autres cellules (ex: frontière, cellules restantes)
    fn get_cell_layers(&self) -> Vec<Vec<Point>> {
        Vec::new()  // Implémentation par défaut : aucune cellule
    }
}