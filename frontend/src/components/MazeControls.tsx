import {ANIMATION, GRID_SIZE} from '../constants/mazeConfig'
import {mazeService} from '../services/mazeService'

interface MazeControlsProps {
  isGenerating: boolean
  isPaused?: boolean
  speed: number
  onSpeedChange: (speed: number) => void
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
  onSpeedChange,
  onGridSizeChange,
  onGenerateInstant,
  onGenerateAnimated,
  onPause,
  onResume,
  onStop,
  disabled = false
}: MazeControlsProps) => {
  const { width: gridWidth, height: gridHeight } = mazeService.getDimensions()

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div className="bg-gray-800 border border-gray-600 rounded-lg p-5">
        <h2 className="text-lg font-medium text-white mb-4">Actions</h2>
        <div className="flex gap-3 flex-wrap justify-center">
          <button
            className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            onClick={onGenerateInstant}
            disabled={isGenerating || disabled}
          >
            Générer instantané
          </button>
          <button
            className="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            onClick={onGenerateAnimated}
            disabled={isGenerating || disabled}
          >
            Générer animé
          </button>
          {(isGenerating || isPaused) && (
            <>
              {isPaused ? (
                <button
                  className="px-4 py-2 bg-emerald-600 text-white rounded-md hover:bg-emerald-700 transition-colors"
                  onClick={onResume}
                >
                  Reprendre
                </button>
              ) : (
                <button
                  className="px-4 py-2 bg-amber-600 text-white rounded-md hover:bg-amber-700 transition-colors"
                  onClick={onPause}
                >
                  Pause
                </button>
              )}
              <button
                className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 transition-colors"
                onClick={onStop}
              >
                Arrêter
              </button>
            </>
          )}
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="md:col-span-2 bg-gray-800 border border-gray-600 rounded-lg p-5">
          <h2 className="text-lg font-medium text-white mb-4">Dimensions</h2>
          <div className="space-y-4">
            <div>
              <div className="flex justify-between items-center mb-2">
                <label htmlFor="grid-width" className="text-sm text-gray-300">Largeur</label>
                <span className="text-sm font-mono text-white bg-gray-700 px-2 py-1 rounded">{gridWidth}</span>
              </div>
              <input
                id="grid-width"
                type="range"
                min={GRID_SIZE.MIN_SIZE}
                max={GRID_SIZE.MAX_SIZE}
                value={gridWidth}
                onChange={(e) => onGridSizeChange(parseInt(e.target.value), gridHeight)}
                disabled={disabled || isGenerating || isPaused}
                className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer disabled:opacity-50"
              />
            </div>
            <div>
              <div className="flex justify-between items-center mb-2">
                <label htmlFor="grid-height" className="text-sm text-gray-300">Hauteur</label>
                <span className="text-sm font-mono text-white bg-gray-700 px-2 py-1 rounded">{gridHeight}</span>
              </div>
              <input
                id="grid-height"
                type="range"
                min={GRID_SIZE.MIN_SIZE}
                max={GRID_SIZE.MAX_SIZE}
                value={gridHeight}
                onChange={(e) => onGridSizeChange(gridWidth, parseInt(e.target.value))}
                disabled={disabled || isGenerating || isPaused}
                className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer disabled:opacity-50"
              />
            </div>
            <div className="pt-3 border-t border-gray-600">
              <p className="text-sm text-gray-300 mb-3">Tailles prédéfinies</p>
              <div className="grid grid-cols-4 gap-2">
                {[20, 30, 50, 100].map(size => (
                  <button
                    key={size}
                    onClick={() => onGridSizeChange(size, size)}
                    disabled={disabled || isGenerating || isPaused}
                    className={`px-3 py-2 text-sm rounded border transition-colors disabled:opacity-50 disabled:cursor-not-allowed ${
                      gridWidth === size && gridHeight === size
                        ? 'bg-blue-600 text-white border-blue-600'
                        : 'bg-gray-700 text-gray-200 border-gray-600 hover:bg-gray-600'
                    }`}
                  >
                    {size}×{size}
                  </button>
                ))}
              </div>
            </div>
          </div>
        </div>

        <div className="bg-gray-800 border border-gray-600 rounded-lg p-5">
          <h2 className="text-lg font-medium text-white mb-4">Vitesse</h2>
          <div>
            <div className="flex justify-between items-center mb-2">
              <label htmlFor="speed-slider" className="text-sm text-gray-300">Animation</label>
              <span className="text-sm font-mono text-white bg-gray-700 px-2 py-1 rounded">{speed}%</span>
            </div>
            <input
              id="speed-slider"
              type="range"
              min={ANIMATION.MIN_SPEED}
              max={ANIMATION.MAX_SPEED}
              value={speed}
              onChange={(e) => onSpeedChange(parseInt(e.target.value))}
              disabled={disabled}
              className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer disabled:opacity-50"
            />
            <div className="flex justify-between text-xs text-gray-400 mt-2">
              <span>Lent</span>
              <span>Rapide</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}