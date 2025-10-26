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
  CURRENT_CELL: 'rgba(255, 0, 0, 0.4)',
} as const

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