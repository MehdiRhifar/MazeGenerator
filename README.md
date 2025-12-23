# Maze Generator

Générateur de labyrinthes aléatoires avec backend Rust/WASM et frontend React.

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

## Déploiement sur Vercel

Le déploiement sur Vercel se fait avec une compilation manuelle du WASM avant le push.

### Étapes de déploiement

1. **Compiler le WASM localement** :
   ```bash
   npm run build:wasm
   ```
   Cela compile le backend Rust en WASM dans le dossier `backend/pkg/` et supprime automatiquement le `.gitignore` créé par wasm-pack.

2. **Commiter les fichiers WASM** :
   ```bash
   git add backend/pkg/
   git add .
   git commit -m "Build WASM for deployment"
   ```

3. **Push vers GitHub** :
   ```bash
   git push
   ```

4. **Déployer sur Vercel** :
   - Connecte ton repo GitHub à Vercel
   - Vercel utilisera automatiquement la configuration de `vercel.json`
   - Le build se fera uniquement pour le frontend (le WASM est déjà compilé)

### Pourquoi cette approche ?

- Vercel n'a pas Rust/Cargo installé par défaut
- La compilation du WASM en local permet un déploiement plus rapide
- Les fichiers WASM compilés sont versionnés dans Git pour Vercel

**Note importante** : wasm-pack crée automatiquement un `.gitignore` dans `pkg/` qui ignore tous les fichiers. Le script `build:wasm` supprime automatiquement ce fichier pour permettre le versionnement des fichiers WASM.

### Configuration Vercel

Le fichier `vercel.json` est déjà configuré pour :
- Builder uniquement le frontend (`buildCommand: "cd frontend && npm install && npm run build"`)
- Utiliser le dossier `frontend/dist` comme sortie
- Le WASM pré-compilé est dans `backend/pkg/` et sera utilisé par le frontend

## Structure du projet

```
.
├── backend/           # Backend Rust compilé en WASM
│   ├── src/          # Code source Rust
│   └── pkg/          # WASM compilé (versionné pour Vercel)
├── frontend/         # Frontend React + TypeScript
│   └── src/
└── vercel.json       # Configuration Vercel
```
