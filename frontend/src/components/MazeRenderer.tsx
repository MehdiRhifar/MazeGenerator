import { useRef, useEffect, forwardRef, useImperativeHandle } from 'react'
import { MazeGenerator, WallChange, WallType } from '../../../backend/pkg'
import { MAZE_CONFIG, COLORS, LAYER_COLORS } from '../constants/mazeConfig'

interface MazeRendererProps {
  mazeGenerator: MazeGenerator | null
  dimensionsKey?: number
}

export interface MazeRendererRef {
  drawFullGrid: () => void
  deleteWallChanges: (changes: WallChange[]) => void
  addWallChanges: (changes: WallChange[]) => void
  drawCell: (x: number, y: number, color: string) => void
  drawCurrentCell: () => void
}

export const MazeRenderer = forwardRef<MazeRendererRef, MazeRendererProps>(
  ({ mazeGenerator, dimensionsKey }, ref) => {
    const canvasRef = useRef<HTMLCanvasElement>(null)
    const lastLayersRef = useRef<Array<Array<{ x: number; y: number }>>>([])


    const drawFullGrid = () => {
      const canvas = canvasRef.current
      if (!canvas || !mazeGenerator) return

      const ctx = canvas.getContext('2d')
      if (!ctx) return

      ctx.clearRect(0, 0, canvas.width, canvas.height)

      // Draw grid background
      ctx.fillStyle = COLORS.BACKGROUND
      ctx.fillRect(0, 0, canvas.width, canvas.height)

      // Draw grid lines (décalées pour laisser place aux murs extérieurs)
      ctx.strokeStyle = COLORS.GRID_LINES
      ctx.lineWidth = 1

      const offset = MAZE_CONFIG.OFFSET
      const gridWidth = mazeGenerator.get_grid_width()
      const gridHeight = mazeGenerator.get_grid_height()
      
      for (let x = 0; x <= gridWidth; x++) {
        ctx.beginPath()
        ctx.moveTo(x * MAZE_CONFIG.CELL_SIZE + offset, offset)
        ctx.lineTo(x * MAZE_CONFIG.CELL_SIZE + offset, gridHeight * MAZE_CONFIG.CELL_SIZE + offset)
        ctx.stroke()
      }
      for (let y = 0; y <= gridHeight; y++) {
        ctx.beginPath()
        ctx.moveTo(offset, y * MAZE_CONFIG.CELL_SIZE + offset)
        ctx.lineTo(gridWidth * MAZE_CONFIG.CELL_SIZE + offset, y * MAZE_CONFIG.CELL_SIZE + offset)
        ctx.stroke()
      }

      // Draw walls selon la convention Rust
      ctx.strokeStyle = COLORS.WALLS
      ctx.lineWidth = MAZE_CONFIG.WALL_SIZE

      // Murs extérieurs (bordures du labyrinthe) - Rectangle fermé
      const canvasGridWidth = gridWidth * MAZE_CONFIG.CELL_SIZE
      const canvasGridHeight = gridHeight * MAZE_CONFIG.CELL_SIZE
      
      ctx.strokeRect(offset, offset, canvasGridWidth, canvasGridHeight)

      // Murs intérieurs - vertical_walls[i] = mur à droite de la cellule i
      for (let y = 0; y < gridHeight; y++) {
        for (let x = 0; x < gridWidth; x++) {
          if (mazeGenerator.has_vertical_wall(x, y)) {
            drawWall(x, y, WallType.Vertical, ctx)
          }
        }
      }

      // Murs intérieurs - horizontal_walls[i] = mur en bas de la cellule i
      for (let y = 0; y < gridHeight; y++) {
        for (let x = 0; x < gridWidth; x++) {
          if (mazeGenerator.has_horizontal_wall(x, y)) {
            drawWall(x, y, WallType.Horizontal, ctx)
          }
        }
      }
    }

    // Fonction utilitaire pour calculer les coordonnées d'un mur
    /**
     * Permet de savoir les coordonners à utiliser pour écrire un mur / le supprimer.
     * Pour la création, on ajoute la moitier de la largeur du mur avant et après pour faire le lien avec les autres murs
     * Pour la suppression, on retire cette partie pour justement ne pas supprimer la partie de l'autre mur.
     */
    const getWallCoordinates = (x: number, y: number, wallType: WallType, forDeletion = false) => {
      const halfWall = MAZE_CONFIG.WALL_SIZE / 2
      
      if (wallType == WallType.Vertical) {
        const wallX = (x + 1) * MAZE_CONFIG.CELL_SIZE + MAZE_CONFIG.OFFSET
        return {
          x1: wallX,
          y1: y * MAZE_CONFIG.CELL_SIZE + MAZE_CONFIG.OFFSET + (forDeletion ? halfWall : -halfWall),
          x2: wallX,
          y2: (y + 1) * MAZE_CONFIG.CELL_SIZE + MAZE_CONFIG.OFFSET + (forDeletion ? -halfWall : halfWall)
        }
      } else {
        const wallY = (y + 1) * MAZE_CONFIG.CELL_SIZE + MAZE_CONFIG.OFFSET
        return {
          x1: x * MAZE_CONFIG.CELL_SIZE + MAZE_CONFIG.OFFSET + (forDeletion ? halfWall : -halfWall),
          y1: wallY,
          x2: (x + 1) * MAZE_CONFIG.CELL_SIZE + MAZE_CONFIG.OFFSET + (forDeletion ? -halfWall : halfWall),
          y2: wallY
        }
      }
    }

    // Fonction générique pour dessiner un mur avec une couleur donnée
    const drawWallWithColor = (x: number, y: number, wallType: WallType, color: string, ctx: CanvasRenderingContext2D, forDeletion = false) => {
      const coords = getWallCoordinates(x, y, wallType, forDeletion)
      
      ctx.strokeStyle = color
      ctx.lineWidth = MAZE_CONFIG.WALL_SIZE
      ctx.beginPath()
      ctx.moveTo(coords.x1, coords.y1)
      ctx.lineTo(coords.x2, coords.y2)
      ctx.stroke()
    }

    const drawWall = (x: number, y: number, wallType: WallType, ctx: CanvasRenderingContext2D) => {
      drawWallWithColor(x, y, wallType, COLORS.WALLS, ctx)
    }

    const deleteWallChanges = (changes: WallChange[]) => {
      const canvas = canvasRef.current
      if (!canvas || !mazeGenerator) return

      const ctx = canvas.getContext('2d')
      if (!ctx) return

      // Effacer les murs en les dessinant avec la couleur de fond (coordonnées réduites)
      changes.forEach((change) => {
        drawWallWithColor(change.x, change.y, change.wall_type, COLORS.BACKGROUND, ctx, true)
      })
    }

    const addWallChanges = (changes: WallChange[]) => {
      const canvas = canvasRef.current
      if (!canvas || !mazeGenerator) return

      const ctx = canvas.getContext('2d')
      if (!ctx) return

      // Ajouter les murs en les dessinant avec la couleur des murs
      changes.forEach((change) => {
        drawWallWithColor(change.x, change.y, change.wall_type, COLORS.WALLS, ctx, false)
      })
    }

    const drawCell = (x: number, y: number, color: string) => {
      const canvas = canvasRef.current
      if (!canvas) return

      const ctx = canvas.getContext('2d')
      if (!ctx) return

      const halfWall = MAZE_CONFIG.WALL_SIZE / 2
      
      // Dessiner le fond de la cellule en évitant les murs
      ctx.fillStyle = color
      ctx.fillRect(
        x * MAZE_CONFIG.CELL_SIZE + MAZE_CONFIG.OFFSET + halfWall,
        y * MAZE_CONFIG.CELL_SIZE + MAZE_CONFIG.OFFSET + halfWall,
        MAZE_CONFIG.CELL_SIZE - MAZE_CONFIG.WALL_SIZE,
        MAZE_CONFIG.CELL_SIZE - MAZE_CONFIG.WALL_SIZE
      )
    }

    const drawCurrentCell = () => {
      if (!mazeGenerator) return

      const cellLayers = mazeGenerator.get_cell_layers()
      const lastLayers = lastLayersRef.current

      // Convertir les nouveaux layers en structure utilisable
      const newLayers: Array<Array<{ x: number; y: number }>> = []
      const newLayerSets: Array<Set<string>> = []

      // Préparer les nouveaux layers
      for (let layerIndex = 0; layerIndex < cellLayers.length; layerIndex++) {
        const layer = cellLayers[layerIndex] as Array<{ x: number; y: number }>
        const newLayerSet = new Set<string>()
        const newLayerCells: Array<{ x: number; y: number }> = []

        for (let i = 0; i < layer.length; i++) {
          const cell = layer[i] as any
          if (cell && typeof cell.x === 'number' && typeof cell.y === 'number') {
            const key = `${cell.x},${cell.y}`
            newLayerSet.add(key)
            newLayerCells.push({ x: cell.x, y: cell.y })
          }
        }

        newLayers.push(newLayerCells)
        newLayerSets.push(newLayerSet)
      }

      // Fonction helper : trouver la couleur à afficher pour une cellule donnée
      // Parcourt les layers du plus haut (priorité) au plus bas
      const getCellColor = (x: number, y: number, maxLayer: number): string => {
        const key = `${x},${y}`
        for (let i = maxLayer; i >= 0; i--) {
          if (newLayerSets[i] && newLayerSets[i].has(key)) {
            return LAYER_COLORS[i] || LAYER_COLORS[0]
          }
        }
        return COLORS.BACKGROUND
      }

      // Traiter chaque layer pour détecter les changements
      for (let layerIndex = 0; layerIndex < Math.max(cellLayers.length, lastLayers.length); layerIndex++) {
        const lastLayerCells = lastLayers[layerIndex] || []
        const newLayerCells = newLayers[layerIndex] || []

        // Créer un Set des anciennes cellules
        const lastLayerSet = new Set<string>()
        lastLayerCells.forEach(cell => {
          lastLayerSet.add(`${cell.x},${cell.y}`)
        })

        const currentLayerSet = newLayerSets[layerIndex] || new Set<string>()
        const color = LAYER_COLORS[layerIndex] || LAYER_COLORS[0]

        // Dessiner les nouvelles cellules de ce layer
        newLayerCells.forEach(cell => {
          const key = `${cell.x},${cell.y}`
          if (!lastLayerSet.has(key)) {
            drawCell(cell.x, cell.y, color)
          }
        })

        // Pour les cellules qui ont disparu de ce layer, redessiner avec la bonne couleur
        lastLayerCells.forEach(cell => {
          const key = `${cell.x},${cell.y}`
          if (!currentLayerSet.has(key)) {
            // Trouver quelle couleur devrait avoir cette cellule maintenant
            const newColor = getCellColor(cell.x, cell.y, layerIndex - 1)
            drawCell(cell.x, cell.y, newColor)
          }
        })
      }

      lastLayersRef.current = newLayers
    }

    useImperativeHandle(ref, () => ({
      drawFullGrid,
      deleteWallChanges,
      addWallChanges,
      drawCell,
      drawCurrentCell,
    }))

    useEffect(() => {
      if (mazeGenerator) {
        drawFullGrid()
        lastLayersRef.current = []
      }
    }, [mazeGenerator, dimensionsKey])

    // Calculer les dimensions du canvas basées sur le générateur
    const canvasWidth = mazeGenerator 
      ? mazeGenerator.get_grid_width() * MAZE_CONFIG.CELL_SIZE + 2 * MAZE_CONFIG.OFFSET
      : MAZE_CONFIG.CELL_SIZE * 30 + 2 * MAZE_CONFIG.OFFSET
    const canvasHeight = mazeGenerator 
      ? mazeGenerator.get_grid_height() * MAZE_CONFIG.CELL_SIZE + 2 * MAZE_CONFIG.OFFSET
      : MAZE_CONFIG.CELL_SIZE * 30 + 2 * MAZE_CONFIG.OFFSET

    return (
      <div className="flex justify-center my-8">
        <canvas
          ref={canvasRef}
          width={canvasWidth}
          height={canvasHeight}
          className="border-2 border-gray-300 rounded-lg bg-white shadow-sm max-w-full max-h-[70vh]"
        />
      </div>
    )
  }
)