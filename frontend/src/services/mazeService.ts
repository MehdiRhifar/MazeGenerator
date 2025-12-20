import init, { MazeGenerator, AlgorithmKind, WallChange } from '../../../backend/pkg/pathfinding.js'
import { GRID_SIZE } from '../constants/mazeConfig'

interface GenerationStepResult {
  isFinished: boolean;
  changes: WallChange[];
}

// Simple service class instead of complex hook
export class MazeService {
  private generator: MazeGenerator | null = null
  private isInitialized = false
  private animationId: number | null = null

  async initialize() {
    if (this.isInitialized) return this.generator

    await init()
    this.generator = new MazeGenerator(GRID_SIZE.DEFAULT_WIDTH, GRID_SIZE.DEFAULT_HEIGHT)
    this.generator.clear_grid()
    this.isInitialized = true
    return this.generator
  }

  generateInstant(algorithm: AlgorithmKind) {
    if (!this.generator) return
    this.stopAnimation()
    this.generator.generate_maze(algorithm)
  }

  prepareGeneration(algorithm: AlgorithmKind) {
    if (!this.generator) return
    this.stopAnimation()
    this.generator.start_generation(algorithm)
  }

  startAnimation(onStep: (changes: WallChange[]) => void, onComplete: () => void, getCurrentSpeed: () => number) {
    if (!this.generator) return
    
    const animate = () => {
      if (!this.generator) return
      const stepResult = this.generator.generation_step_with_changes() as GenerationStepResult
      onStep(stepResult.changes)

      if (stepResult.isFinished) {
        onComplete()
        this.animationId = null
        return
      }

      const currentDelay = Math.abs(100 - getCurrentSpeed())
      this.animationId = window.setTimeout(animate, currentDelay)
    }

    animate()
  }

  pauseAnimation() {
    if (this.animationId) {
      clearTimeout(this.animationId)
      this.animationId = null
    }
  }

  stopAnimation() {
    this.pauseAnimation()
  }

  getCurrentPosition() {
    if (!this.generator) return
    return this.generator.get_current_cell()
  }

  resizeGrid(width: number, height: number) {
    if (!this.generator) return
    this.stopAnimation()
    this.generator.resize_grid(width, height)
  }

  getDimensions() {
    if (!this.generator) return { width: 0, height: 0 }
    return {
      width: this.generator.get_grid_width(),
      height: this.generator.get_grid_height()
    }
  }
  
  getGenerator() {
    return this.generator
  }
}

// Instance globale
export const mazeService = new MazeService()