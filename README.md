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

## Déploiement sur Vercel via GitHub Actions

Le projet utilise GitHub Actions pour compiler le WASM et déployer automatiquement sur Vercel.

### Configuration initiale (à faire une seule fois)

#### 1. Obtenir les tokens Vercel

**VERCEL_TOKEN** :
1. Va sur [Vercel Settings → Tokens](https://vercel.com/account/tokens)
2. Clique sur "Create Token"
3. Nomme-le "GitHub Actions"
4. Copie le token (tu ne pourras plus le voir après)

**VERCEL_ORG_ID** et **VERCEL_PROJECT_ID** :
1. Installe Vercel CLI localement : `npm i -g vercel`
2. Dans le dossier `frontend/`, lance : `vercel link`
3. Suis les instructions pour lier ton projet
4. Un dossier `.vercel` sera créé avec un fichier `project.json`
5. Récupère les IDs :
   ```bash
   cat frontend/.vercel/project.json
   ```
   Tu verras quelque chose comme :
   ```json
   {
     "orgId": "team_xxxxxxxxxxxxx",
     "projectId": "prj_xxxxxxxxxxxxx"
   }
   ```

#### 2. Ajouter les secrets dans GitHub

1. Va sur ton repo GitHub → **Settings** → **Secrets and variables** → **Actions**
2. Clique sur "New repository secret"
3. Ajoute ces 3 secrets :
   - `VERCEL_TOKEN` : le token créé à l'étape 1
   - `VERCEL_ORG_ID` : le `orgId` du fichier `project.json`
   - `VERCEL_PROJECT_ID` : le `projectId` du fichier `project.json`

### Workflow de déploiement

Une fois configuré, le déploiement est **entièrement automatique** :

1. Tu modifies le code et commit
2. Tu push vers `master`
3. GitHub Actions se déclenche automatiquement :
   - Compile le backend Rust en WASM
   - Build le frontend
   - Déploie sur Vercel
4. Ton site est mis à jour ! 🚀

Tu peux suivre le déploiement dans l'onglet **Actions** de ton repo GitHub.

### Preview deployments

Les Pull Requests déclenchent aussi un déploiement de preview automatique pour tester avant de merger.

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

### GitHub Actions

Le workflow `.github/workflows/deploy.yml` :
- Se déclenche sur les push vers `master` ou les Pull Requests
- Installe Rust et wasm-pack
- Compile le WASM (qui n'est pas versionné dans Git)
- Build le frontend avec le WASM compilé
- Déploie sur Vercel via Vercel CLI

### Pourquoi cette approche ?

- ✅ Pas de fichiers générés dans Git (propre)
- ✅ Build automatique à chaque push
- ✅ Preview deployments pour les PR
- ✅ Cache Cargo pour des builds rapides
- ✅ Workflow professionnel et reproductible
