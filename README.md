# Maze Generator

Générateur de labyrinthes aléatoires avec backend Rust/WASM et frontend React.


[https://maze-generator-mehdi-rhifar.vercel.app]([https://link-url-here.org](https://maze-generator-mehdi-rhifar.vercel.app))

## Algorithmes disponibles

- Recursive Backtracking
- Prim
- Kruskal
- Wilson
- Recursive Division

## Développement local

### Prérequis

- Node.js et npm
- Rust et Cargo
- wasm-pack

### Lancer le projet en développement

1. **Compiler le backend WASM** :
   ```bash
   npm run build:wasm
   ```

2. **Lancer le frontend** :
   ```bash
   npm run dev:frontend
   ```

Ou utiliser le mode watch pour le WASM :
```bash
npm run dev:wasm  # Terminal 1
npm run dev:frontend  # Terminal 2
```

## Architecture

```
.
├── backend/              # Backend Rust compilé en WASM
│   ├── src/             # Code source Rust
│   ├── pkg/             # WASM compilé (généré, ignoré par Git)
│   └── target/          # Artefacts Cargo (ignoré par Git)
├── frontend/            # Frontend React + TypeScript
│   └── src/
└── .github/
    └── workflows/
        └── deploy.yml   # GitHub Actions pour CI/CD
```

## Comment ça fonctionne ?

### Le processus de build

1. **Rust → WASM** : Le code Rust dans `backend/src/` est compilé en WebAssembly
2. **wasm-pack** : Génère les bindings JavaScript dans `backend/pkg/`
3. **Frontend** : Importe le WASM depuis `backend/pkg/pathfinding.js`
4. **Vite** : Bundle tout pour la production dans `frontend/dist/`
