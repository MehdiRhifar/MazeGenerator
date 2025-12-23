export const MAZE_CONFIG = {
  GRID_WIDTH: 30,
  GRID_HEIGHT: 30,
  CELL_SIZE: 12,
  WALL_SIZE: 4,
  OFFSET: 6
} as const

export const COLORS = {
  BACKGROUND: '#f5f5f5',
  GRID_LINES: '#e0e0e0',
  WALLS: '#1a1a1a',
} as const

// Couleurs des layers (ordre = Layer 0, Layer 1, Layer 2, ...)
export const LAYER_COLORS = [
  'rgba(59, 130, 246, 0.85)',   // Layer 1 : Bleu - Le chemin de la marche aléatoire
  'rgba(34, 197, 94, 0.80)',    // Layer 0 : Vert - Cellules dans le labyrinthe (le "but")
] as const

export const ANIMATION = {
  MIN_SPEED: 1,
  MAX_SPEED: 100,
  DEFAULT_SPEED: 50,
} as const

export const GRID_SIZE = {
  MIN_SIZE: 5,
  MAX_SIZE: 200,
  DEFAULT_WIDTH: 30,
  DEFAULT_HEIGHT: 30,
} as const