# Pokemon Lite 🎮

Un jeu Pokémon simplifié développé en Rust, mettant en avant les principes de programmation orientée objet, la gestion des traits et les systèmes graphiques.

### Dépendances principales

- **[Macroquad](https://docs.rs/macroquad/latest/macroquad/)** (v0.4) - Framework graphique 2D
  - Gestion des textures et rendu des sprites
  - Gestion des événements clavier
  - Système de fenêtre et de rendu

- **[Image](https://docs.rs/image/latest/image/)** (v0.25) - Traitement d'images
  - Chargement et manipulation de fichiers PNG
  - Redimensionnement des sprites (image::imageops::resize)
  - Manipulation des pixels RGBA pour la transparence

## 🏗️ Architecture

### Structure des modules

```
pokemon_lite/
├── src/
│   ├── main.rs                 # Boucle de jeu principale
│   ├── player.rs               # Système de mouvement du joueur
│   ├── pokemon.rs              # Logique backend des Pokémon (trait + implémentations)
│   ├── dresseur.rs             # Gestion de l'équipe du dresseur
│   ├── combat.rs               # Système de combat (en cours de développement)
│   ├── pokemon_renderer.rs     # Liaison backend/graphique pour les Pokémon
│   ├── trainer_animations.rs   # Gestion des animations du dresseur
│   ├── sprite_loader.rs        # Chargement et traitement des sprites
│   ├── pokedex.rs              # Gestion des sprites Pokémon
│   ├── graphics.rs             # Fonctions graphiques (UI)
│   └── lib.rs                  # Exports des modules
└── texture/
    ├── pokemon/                # Sprites des Pokémon (32×32px)
    ├── dresseur/               # Animations du dresseur (16×24px)
    └── Game Boy Square.png     # Map de fond (1064×1007px)
```
Un dresseur possède des pokémons dans un Vecteur dynamique et un Pokemon possèdes les différentes méthodes : attaquer, prendre_degats, est_vivant, etc... pour lancer un combat. 

### Concepts clés

#### 1. **Traits Rust**
- `Pokemon` : Trait implémenté par Flamby, Aquali, Florizarre

#### 2. **Système d'animation**
- `Direction` enum : Up, Down, Left, Right
- `AnimationState` enum : Stop, RunningLeft, RunningRight
- Alternance des frames toutes les 0.5 secondes

#### 3. **Traitement d'images**
- Suppression automatique des fonds colorés (blanc, bleu)
- Redimensionnement des sprites pour correspondre à la map
- Conversion PNG → RgbaImage → Texture2D Macroquad

#### 4. **Système de mouvement**
- Déplacement fluide pixel par pixel (1 pixel/50ms)
- Cooldown pour éviter les mouvements trop rapides
- Vérification des limites de la map

## 🎮 Contrôles

| Touche | Action |
|--------|--------|
| ↑ / ↓ / ← / → | Mouvement du dresseur |
| E | Quitter le jeu |

## 📦 Dépendances en détail

### Macroquad
Utilisé pour :
- Rendu des textures et sprites
- Détection des entrées clavier
- Gestion de la fenêtre de jeu
- Calcul du FPS et du delta time

### Image
Utilisé pour :
- `ImageReader::open()` : Charger les fichiers PNG
- `DynamicImage::to_rgba8()` : Convertir en format RGBA
- `imageops::resize()` : Redimensionner les sprites
- Manipulation directe des pixels pour la transparence

## 📋 Notes de développement

- **Taille TILE_SIZE** : 1 pixel (mouvement granulaire)
- **Cooldown de mouvement** : 0.05 secondes (50ms)
- **Taille du Pokémon affiché** : 64×64 pixels
- **Taille du dresseur affiché** : 16×16 pixels
- **Taille de la map** : 1064×1007 pixels

## 👨‍💻 Auteur : Matthieu Tremblay

Projet développé à titre éducatif pour l'école ESEO dans le cours de Rust.