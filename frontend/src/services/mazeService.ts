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

    const MAX_STEPS_PER_FRAME = 150 // Limite pour garder l'UI fluide

    let lastTime = performance.now()
    let stepDebt = 0

    const animate = (currentTime: number) => {
      if (!this.generator) return

      const deltaTime = currentTime - lastTime
      lastTime = currentTime

      // Calcul de la vitesse : stepsPerSecond = 1.07 ^ speed
      const speed = getCurrentSpeed()
      const stepsPerSecond = Math.pow(1.07, speed)

      // Accumulation des steps + limite de sécurité
      stepDebt += stepsPerSecond * (deltaTime / 1000)
      const stepsToExecute = Math.min(Math.floor(stepDebt), MAX_STEPS_PER_FRAME)
      stepDebt -= stepsToExecute

      const allChanges: WallChange[] = []
      let isFinished = false

      for (let i = 0; i < stepsToExecute; i++) {
        const stepResult = this.generator.generation_step_with_changes() as GenerationStepResult
        allChanges.push(...stepResult.changes)

        if (stepResult.isFinished) {
          isFinished = true
          break
        }
      }

      if (stepsToExecute > 0) {
        onStep(allChanges)
      }

      if (isFinished) {
        onComplete()
        this.animationId = null
        return
      }

      this.animationId = window.requestAnimationFrame(animate)
    }

    this.animationId = window.requestAnimationFrame(animate)
  }

  pauseAnimation() {
    if (this.animationId !== null) {
      window.cancelAnimationFrame(this.animationId)
      this.animationId = null
    }
  }

  stopAnimation() {
    this.pauseAnimation()
  }

  getCellLayers() {
    if (!this.generator) return []
    const layers = this.generator.get_cell_layers()
    // Convertir le tableau JavaScript en tableau TypeScript
    return Array.from(layers as any)
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