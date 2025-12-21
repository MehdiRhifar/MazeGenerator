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

    let frameCount = 0

    const animate = () => {
      if (!this.generator) return

      const speed = getCurrentSpeed()

      // Logique avec deux modes :
      // Mode lent (speed 1-80) : 1 step toutes les N frames
      // Mode rapide (speed 81-100) : N steps par frame

      let stepsPerFrame = 0

      if (speed <= 90) {
        // Mode lent : attendre plusieurs frames avant de faire 1 step
        // speed=1 → 30 frames (~2 steps/sec)
        // speed=90 → 1 frame (~60 steps/sec)
        const framesPerStep = Math.max(1, Math.round(31 - speed * 0.375))

        frameCount++
        if (frameCount >= framesPerStep) {
          stepsPerFrame = 1
          frameCount = 0
        }
      } else {
        // Mode rapide : faire plusieurs steps par frame
        // speed=91 → 1 step/frame (~60 steps/sec)
        // speed=100 → 10 steps/frame (~600 steps/sec)
        stepsPerFrame = Math.round((speed - 90))
        frameCount = 0
      }

      const allChanges: WallChange[] = []
      let isFinished = false

      // Exécuter les steps
      for (let i = 0; i < stepsPerFrame; i++) {
        const stepResult = this.generator.generation_step_with_changes() as GenerationStepResult
        allChanges.push(...stepResult.changes)

        if (stepResult.isFinished) {
          isFinished = true
          break
        }
      }

      // Redessiner seulement si on a fait des steps
      if (stepsPerFrame > 0) {
        onStep(allChanges)
      }

      if (isFinished) {
        onComplete()
        this.animationId = null
        return
      }

      // Utiliser requestAnimationFrame au lieu de setTimeout
      this.animationId = window.requestAnimationFrame(animate)
    }

    animate()
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