import {ANIMATION, GRID_SIZE} from '../constants/mazeConfig'
import {mazeService} from '../services/mazeService'
import { AlgorithmKind } from '../../../backend/pkg'

interface MazeControlsProps {
  isGenerating: boolean
  isPaused?: boolean
  speed: number
  selectedAlgorithm: AlgorithmKind
  onSpeedChange: (speed: number) => void
  onAlgorithmChange: (algorithm: AlgorithmKind) => void
  onGridSizeChange: (width: number, height: number) => void
  onGenerateInstant: () => void
  onGenerateAnimated: () => void
  onPause: () => void
  onResume: () => void
  onStop: () => void
  disabled?: boolean
}

export const MazeControls = ({
  isGenerating,
  isPaused = false,
  speed,
  selectedAlgorithm,
  onSpeedChange,
  onAlgorithmChange,
  onGridSizeChange,
  onGenerateInstant,
  onGenerateAnimated,
  onPause,
  onResume,
  onStop,
  disabled = false
}: MazeControlsProps) => {
  const { width: gridWidth, height: gridHeight } = mazeService.getDimensions()

  const algorithms = [
    {
      value: AlgorithmKind.Backtracking,
      label: 'Recursive Backtracking',
      description: 'Parcours en profondeur avec retour arrière',
      tooltip: 'Commence par une cellule et explore en profondeur jusqu\'à atteindre un cul-de-sac, puis revient en arrière. Crée de longs passages tortueux avec peu d\'embranchements.'
    },
    {
      value: AlgorithmKind.Prim,
      label: 'Prim (Randomized)',
      description: 'Expansion progressive depuis un point de départ',
      tooltip: 'Commence par une cellule et ajoute aléatoirement des cellules adjacentes. Génère beaucoup de branches courtes et un aspect plus organique.'
    },
    {
      value: AlgorithmKind.Kruskal,
      label: 'Kruskal',
      description: 'Union-Find pour connecter des ensembles',
      tooltip: 'Supprime aléatoirement des murs entre cellules non connectées. Utilise l\'algorithme Union-Find pour créer des passages sinueux et longs.'
    },
    {
      value: AlgorithmKind.Wilson,
      label: 'Wilson',
      description: 'Marches aléatoires effaçant les boucles',
      tooltip: 'Effectue des marches aléatoires jusqu\'à atteindre une cellule visitée, puis efface les boucles. Génère des labyrinthes parfaitement uniformes (unbiased).'
    },
    {
      value: AlgorithmKind.RecursiveDivision,
      label: 'Recursive Division',
      description: 'Division récursive de l\'espace',
      tooltip: 'Commence avec un espace vide et ajoute des murs récursivement en laissant des passages. Crée de longues lignes droites et un aspect chambré.'
    },
  ]

  return (
    <div className="space-y-4">
      {/* Section Actions */}
      <div className="bg-gray-800 border border-gray-600 rounded-lg p-4 shadow-lg">
        <h2 className="text-base font-medium text-white mb-3">Actions</h2>
        <div className="flex flex-col gap-2">
          <button
            className="w-full px-4 py-2.5 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-all hover:shadow-lg disabled:opacity-50 disabled:cursor-not-allowed text-sm font-medium"
            onClick={onGenerateInstant}
            disabled={isGenerating || disabled}
          >
            ⚡ Générer instantané
          </button>
          <button
            className="w-full px-4 py-2.5 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-all hover:shadow-lg disabled:opacity-50 disabled:cursor-not-allowed text-sm font-medium"
            onClick={onGenerateAnimated}
            disabled={isGenerating || disabled}
          >
            🎬 Générer animé
          </button>
          {(isGenerating || isPaused) && (
            <div className="flex gap-2">
              {isPaused ? (
                <button
                  className="flex-1 px-4 py-2.5 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-all hover:shadow-lg text-sm font-medium"
                  onClick={onResume}
                >
                  ▶️ Reprendre
                </button>
              ) : (
                <button
                  className="flex-1 px-4 py-2.5 bg-amber-600 text-white rounded-lg hover:bg-amber-700 transition-all hover:shadow-lg text-sm font-medium"
                  onClick={onPause}
                >
                  ⏸️ Pause
                </button>
              )}
              <button
                className="flex-1 px-4 py-2.5 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-all hover:shadow-lg text-sm font-medium"
                onClick={onStop}
              >
                ⏹️ Arrêter
              </button>
            </div>
          )}
        </div>
      </div>

      {/* Section Algorithmes */}
      <div className="bg-gray-800 border border-gray-600 rounded-lg p-4 shadow-lg">
        <h2 className="text-base font-medium text-white mb-3">Algorithme</h2>
        <div className="space-y-2">
          {algorithms.map((algo) => (
            <button
              key={algo.value}
              onClick={() => onAlgorithmChange(algo.value)}
              disabled={disabled || isGenerating || isPaused}
              title={algo.tooltip}
              className={`w-full text-left px-3 py-2.5 rounded-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed ${
                selectedAlgorithm === algo.value
                  ? 'bg-blue-600 text-white border-2 border-blue-400 shadow-lg'
                  : 'bg-gray-700 text-gray-200 border-2 border-gray-600 hover:bg-gray-600 hover:border-gray-500'
              }`}
            >
              <div className="font-semibold text-sm">{algo.label}</div>
              <div className="text-xs opacity-90 mt-0.5 leading-snug">{algo.description}</div>
            </button>
          ))}
        </div>
      </div>

      {/* Section Dimensions */}
      <div className="bg-gray-800 border border-gray-600 rounded-lg p-4 shadow-lg">
        <h2 className="text-base font-medium text-white mb-3">Dimensions</h2>
        <div className="space-y-3">
          <div>
            <div className="flex justify-between items-center mb-1.5">
              <label htmlFor="grid-width" className="text-xs text-gray-300 font-medium">Largeur</label>
              <span className="text-xs font-mono text-white bg-gray-700 px-2 py-0.5 rounded font-bold">{gridWidth}</span>
            </div>
            <input
              id="grid-width"
              type="range"
              min={GRID_SIZE.MIN_SIZE}
              max={GRID_SIZE.MAX_SIZE}
              value={gridWidth}
              onChange={(e) => onGridSizeChange(parseInt(e.target.value), gridHeight)}
              disabled={disabled || isGenerating || isPaused}
              className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer disabled:opacity-50 accent-blue-500"
            />
          </div>
          <div>
            <div className="flex justify-between items-center mb-1.5">
              <label htmlFor="grid-height" className="text-xs text-gray-300 font-medium">Hauteur</label>
              <span className="text-xs font-mono text-white bg-gray-700 px-2 py-0.5 rounded font-bold">{gridHeight}</span>
            </div>
            <input
              id="grid-height"
              type="range"
              min={GRID_SIZE.MIN_SIZE}
              max={GRID_SIZE.MAX_SIZE}
              value={gridHeight}
              onChange={(e) => onGridSizeChange(gridWidth, parseInt(e.target.value))}
              disabled={disabled || isGenerating || isPaused}
              className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer disabled:opacity-50 accent-blue-500"
            />
          </div>
          <div className="pt-2 border-t border-gray-600">
            <p className="text-xs text-gray-300 mb-2 font-medium">Presets</p>
            <div className="grid grid-cols-4 gap-1.5">
              {[20, 30, 50, 100].map(size => (
                <button
                  key={size}
                  onClick={() => onGridSizeChange(size, size)}
                  disabled={disabled || isGenerating || isPaused}
                  className={`px-2 py-1.5 text-xs rounded border transition-all disabled:opacity-50 disabled:cursor-not-allowed ${
                    gridWidth === size && gridHeight === size
                      ? 'bg-blue-600 text-white border-blue-600 shadow-md font-bold'
                      : 'bg-gray-700 text-gray-200 border-gray-600 hover:bg-gray-600 hover:border-gray-500'
                  }`}
                >
                  {size}×{size}
                </button>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* Section Vitesse */}
      <div className="bg-gray-800 border border-gray-600 rounded-lg p-4 shadow-lg">
        <h2 className="text-base font-medium text-white mb-3">Vitesse</h2>
        <div>
          <div className="flex justify-between items-center mb-1.5">
            <label htmlFor="speed-slider" className="text-xs text-gray-300 font-medium">Animation</label>
            <span className="text-xs font-mono text-white bg-gray-700 px-2 py-0.5 rounded font-bold">{speed}%</span>
          </div>
          <input
            id="speed-slider"
            type="range"
            min={ANIMATION.MIN_SPEED}
            max={ANIMATION.MAX_SPEED}
            value={speed}
            onChange={(e) => onSpeedChange(parseInt(e.target.value))}
            disabled={disabled}
            className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer disabled:opacity-50 accent-green-500"
          />
          <div className="flex justify-between text-xs text-gray-400 mt-1.5">
            <span>🐌 Lent</span>
            <span>🚀 Rapide</span>
          </div>
        </div>
      </div>
    </div>
  )
}