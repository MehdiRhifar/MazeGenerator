import { useState, useEffect, useRef } from 'react'
import './App.css'
import { mazeService } from './services/mazeService'
import { MazeRenderer, type MazeRendererRef } from './components/MazeRenderer'
import { MazeControls } from './components/MazeControls'
import { MazeGenerator, AlgorithmKind } from '../../backend/pkg'

function App() {
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [isGenerating, setIsGenerating] = useState(false)
  const [isPaused, setIsPaused] = useState(false)
  const [speed, setSpeed] = useState(50)
  const [selectedAlgorithm, setSelectedAlgorithm] = useState<AlgorithmKind>(AlgorithmKind.Backtracking)
  const [generator, setGenerator] = useState<MazeGenerator | null>(null)
  const [dimensionsKey, setDimensionsKey] = useState(0) // Pour forcer le re-render

  const mazeRendererRef = useRef<MazeRendererRef>(null)
  const speedRef = useRef(speed)

  // Initialize WASM
  useEffect(() => {
    mazeService.initialize()
      .then((gen) => {
        setGenerator(gen)
        setLoading(false)
      })
      .catch((err) => {
        setError(`Erreur de chargement: ${err.message}`)
        setLoading(false)
      })
  }, [])

  // Keep speedRef in sync with speed state
  useEffect(() => {
    speedRef.current = speed
  }, [speed])

  const handleInstantGeneration = () => {
    mazeService.generateInstant(selectedAlgorithm)
    mazeRendererRef.current?.drawFullGrid()
  }

  const handleAnimatedGeneration = () => {
    if (!mazeRendererRef.current) return


    mazeService.prepareGeneration(selectedAlgorithm)
    mazeRendererRef.current.drawFullGrid()
    setIsGenerating(true)
    handleResume()
  }

  const handlePause = () => {
    mazeService.pauseAnimation()
    setIsPaused(true)
  }

  const handleResume = () => {
    setIsPaused(false)
    mazeService.startAnimation(
      // onStep - mêmes callbacks qu'au démarrage
      (changes) => {
        // Recursive Division AJOUTE des murs, les autres SUPPRIMENT des murs
        if (selectedAlgorithm === AlgorithmKind.RecursiveDivision) {
          mazeRendererRef.current?.addWallChanges(changes)
        } else {
          mazeRendererRef.current?.deleteWallChanges(changes)
        }
        mazeRendererRef.current?.drawCurrentCell()
      },
      // onComplete
      () => {
        mazeRendererRef.current?.drawFullGrid()
        setIsGenerating(false)
        setIsPaused(false)
      },
      // getCurrentSpeed
      () => speedRef.current
    )
  }

  const handleStop = () => {
    mazeService.stopAnimation()
    setIsGenerating(false)
    setIsPaused(false)
  }

  const handleGridSizeChange = (width: number, height: number) => {
    // Arrêter toute animation en cours
    if (isGenerating) {
      handleStop()
    }
    
    // Redimensionner la grille existante
    mazeService.resizeGrid(width, height)
    
    // Forcer le re-render du MazeRenderer
    setDimensionsKey(prev => prev + 1)
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-white flex items-center justify-center">
        <div className="text-center">
          <div className="w-12 h-12 border-3 border-gray-200 border-t-blue-600 rounded-full animate-spin mx-auto mb-4"></div>
          <p className="text-gray-600 font-medium">Chargement...</p>
        </div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="min-h-screen bg-white flex items-center justify-center">
        <div className="text-center p-8 max-w-md">
          <div className="text-red-600 text-5xl mb-4">⚠️</div>
          <h3 className="text-xl font-semibold text-gray-900 mb-2">Erreur</h3>
          <p className="text-gray-600">{error}</p>
        </div>
      </div>
    )
  }

  return (
    <div className="max-w-6xl mx-auto px-6 py-8">
      <header className="text-center mb-8">
        <h1 className="text-3xl font-bold text-white mb-2">Générateur de Labyrinthe</h1>
        <p className="text-gray-300">5 algorithmes implémentés en Rust/WebAssembly</p>
      </header>

      <MazeControls
        isGenerating={isGenerating}
        isPaused={isPaused}
        speed={speed}
        selectedAlgorithm={selectedAlgorithm}
        onSpeedChange={setSpeed}
        onAlgorithmChange={setSelectedAlgorithm}
        onGridSizeChange={handleGridSizeChange}
        onGenerateInstant={handleInstantGeneration}
        onGenerateAnimated={handleAnimatedGeneration}
        onPause={handlePause}
        onResume={handleResume}
        onStop={handleStop}
        disabled={!generator}
      />

      <MazeRenderer
        ref={mazeRendererRef}
        mazeGenerator={generator}
        dimensionsKey={dimensionsKey}
      />

      <footer className="mt-8 text-center text-sm text-gray-400">
        <p>Utilise les boutons pour générer un labyrinthe, ajuste la taille et la vitesse selon tes préférences.</p>
      </footer>
    </div>
  )
}

export default App