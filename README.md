# Pokémon Lite - Jeu Pokémon en Rust

## 📋 Vue d'ensemble

**Pokémon Lite** est un jeu Pokémon simplifié développé en **Rust** utilisant la bibliothèque graphique **macroquad**. Le jeu met en place un système de combat tour par tour avec un dresseur explorant une carte pour rencontrer et combattre Célèbi, le Pokémon légendaire.

### Fonctionnalités principales
- ✅ Exploration d'une carte interactive
- ✅ Génération asynchrone de potions (thread + mutex)
- ✅ Système de combat tour par tour
- ✅ Rencontres avec Pokémon ennemis
- ✅ Inventaire de potions collectables
- ✅ Animations du dresseur

---

## �️ Architecture et Modules

### 1. **main.rs** - Boucle de jeu principale
**Responsabilité** : Orchestration générale du jeu, gestion d'état et boucle graphique

**Fonctionnalités clés** :
- Initialisation du contexte macroquad
- Boucle de jeu (input → update → render)
- Gestion de la machine d'état (exploration ↔ combat)
- Gestion des collisions (joueur-potions, joueur-Pokémon, joueur-Célèbi)
- Affichage des pop-ups (rencontre, victoire)
- Chargement des textures et ressources

**Dépendances** : 
- `macroquad` - Gestion graphique et input
- Tous les autres modules

### 2. **potion.rs** - Gestion des potions
**Responsabilité** : Définition des données et comportements des potions

**Structures** :
- `Potion { x, y, id, hp_restore }` - Représentation d'une potion sur la carte

**Fonctionnalités** :
- Génération de potions à positions aléatoires
- Détection de collision avec le joueur (distance < 20px)
- Rendu graphique (cercle rouge + anneau jaune)

**Thread-safe** : Non, utilisé uniquement en lecture par le thread principal

### 3. **potion_manager.rs** - Gestion thread-safe des potions
**Responsabilité** : Gérer l'accès concurrent aux potions via un thread de génération

**Concepts Rust critiques** :
- **Arc** (Atomic Reference Counting) - Partage de propriété entre threads
- **Mutex** - Verrouillage exclusif pour accès sécurisé à `Vec<Potion>`
- **Thread spawning** - Création d'un thread détaché qui s'exécute indéfiniment

**Fonctionnalités** :
- `new()` - Initialise Arc<Mutex<Vec<Potion>>> et lance le thread de génération
- `get_potions()` - Acquiert le verrou mutex, clone le vecteur, relâche le verrou
- `collect_potions_at_position()` - Détecte et supprime les potions en collision
- `remove_potion()` - Supprime une potion spécifique par ID

**Comportement du thread** :
```rust
thread::spawn(move || {
    loop {
        thread::sleep(Duration::from_secs(2));  // Génère une potion tous les 2 secondes
        let mut potions_lock = potions.lock().unwrap();
        potions_lock.push(/* nouvelle potion */);
        println!("✨ Nouvelle potion générée...");
    }
});
```

### 4. **pokemon.rs** - Traits et implémentations Pokémon
**Responsabilité** : Définir le comportement commun des Pokémon

**Trait** `Pokemon` :
- `get_nom()` - Nom du Pokémon
- `get_hp()` / `set_hp()` - Gestion des points de vie
- `get_max_hp()` - HP maximum
- `prendre_degats()` - Application de dégâts
- `attaquer()` - Calcul de dégâts d'attaque

**Implémentations** :
- `Flamby` - Pokémon feu (attaque 45 dégâts, 60 HP max)
- `Aquali` - Pokémon eau (attaque 42 dégâts, 65 HP max)

**Traits avancés Rust** :
- Trait objects (`Box<dyn Pokemon>`) pour polymorphisme
- Implémentation manuelle de `Clone` (les traits objects ne sont pas clonables par défaut)
- Gestion de la durée de vie implicite

### 5. **combat.rs** - Système de combat tour par tour
**Responsabilité** : Logique et interface du combat - réalisé en partie par l'IA

**Structure** `CombatState` :
```rust
pub struct CombatState {
    pub pokemon_joueur: Box<dyn Pokemon>,
    pub pokemon_sauvage: Box<dyn Pokemon>,
    pub tour_joueur: bool,
    pub combat_timer: f32,
    pub action_selectionnee: ActionCombat,
    pub messages_combat: Vec<String>,
}
```

**Fonctionnalités** :
- `traiter_input_combat()` - Gère les touches 1-4 (attaque, pokéball, potion, fuite)
- `joueur_attaque()` - Applique les dégâts au Pokémon ennemi
- `sauvage_attaque()` - Inverse les rôles (attaque du Pokémon ennemi)
- `dessiner_interface_combat()` - Rendu UI (barres HP, boutons d'action)
- `est_termine()` - Vérifie la victoire/défaite

**Mécanique** :
- Tour du joueur → 2 secondes d'attente → Tour du Pokémon ennemi → Cycle
- Victoire si HP ennemi ≤ 0

### 7. **inventory.rs** - Gestion de l'inventaire du joueur
**Responsabilité** : Stockage et gestion des Pokémon du joueur

**Fonctionnalités** :
- `add_pokemon()` - Ajoute un Pokémon à l'équipe
- `get_current_pokemon()` - Retourne le Pokémon actif
- `switch_to_next()` - Change de Pokémon

**État actuel** : Partiellement intégré (utilisé mais pas complètement fonctionnel dans le combat)

### 8. **player.rs** - Gestion du dresseur
**Responsabilité** : État et mouvement du joueur

**Fonctionnalités** :
- `move_up/down/left/right()` - Déplacement avec cooldown (30ms entre mouvements)
- `update_animation()` - Cycle d'animation du sprite (4 frames)
- `can_move()` - Vérification du cooldown

**Systèmes avancés Rust** :
- Chronomètre interne pour gestion du cooldown
- Énumération pour direction d'animation

### 9. **trainer_animations.rs** - Gestion des animations du dresseur
**Responsabilité** : Charger et gérer les sprites du dresseur

**Fonctionnalités** :
- `get_frame()` - Retourne la texture correspondant à l'animation actuelle
- Support de 4 directions × 4 frames chacune

### 10. **graphics.rs** - Fonctions utilitaires graphiques
**Responsabilité** : Rendu UI (texte, boîtes, inventaire)

**Fonctionnalités** :
- `draw_ui()` - Affiche info joueur (nom, position, tile)
- Rendu du HUD de jeu

### 11. **dresseur.rs** - Données du dresseur
**Responsabilité** : Définition du personnage joueur

**Données** :
- Nom, position (x, y)
- État animation

### 12. **lib.rs** - Déclaration des modules

---

## 🔧 Bibliothèques externes

### macroquad (0.4.14)
**Utilisation** : Framework graphique 2D complet
- Gestion de la fenêtre et boucle de jeu
- Rendu de textures et formes (cercles, rectangles)
- Système d'input (clavier)
- Chargement d'images PNG

**Caractéristiques avancées utilisées** :
- `DrawTextureParams` - Rendu personnalisé des textures (mise à l'échelle)
- Gestion de la transparence et couleurs (RGBA)

### rand
**Utilisation** : Génération de nombres aléatoires
- Positions aléatoires des potions
- Positionnement des Pokémon ennemis

**Note** : Utilise des APIs dépréciées (`thread_rng`, `gen_range`) - voir section Warnings

---

## ⚠️ Justification des Warnings

### 1. **Deprecated functions in `potion.rs`**
```
warning: use of deprecated function `rand::thread_rng`
Renamed to `rng`
```
**Raison** : Version legacy de la crate `rand` utilisée
**Impact** : Aucun - Le code fonctionne correctement, c'est juste un avertissement de maintenance
**Peut être corrigé** : Mise à jour future de la crate rand vers une version plus récente

### 2. **Unused imports/variables**
```
warning: unused variable: `pokemon_textures`
warning: unused variable: `pokemon`
```
**Raison** : Code préparé pour futures fonctionnalités (sprites Pokémon multiples, etc.)
**Impact** : Aucun - Code optionnel, simplement commenté par les variables préfixées `_`
**Stratégie** : Volontairement gardés car code de base pour extensions futures

### 3. **Unused assignments** (`in_battle` variable)
```
warning: value assigned to `in_battle` is never read
```
**Raison** : Variable de suivi historique, remplacée par `Option<CombatState>`
**Impact** : Peut être supprimée
**Raison de la conservation** : Facilite debug/logging futur

### 4. **Unnecessary mut**
```
warning: variable does not need to be mutable
let mut pokemon_spawner
```
**Raison** : `pokemon_spawner` n'est jamais modifié après initialisation
**Impact** : Très mineure
**Peut être corrigé** : Supprimer le `mut`

**Conclusion** : Tous les warnings sont **non-critiques** et n'impactent pas la fonctionnalité du jeu. Le code compile et s'exécute sans erreur.

---

## 🎯 Concepts Rust avancés utilisés

### 1. **Arc<Mutex<T>>** pour concurrence thread-safe
- Permet au thread de génération de potions et au thread principal de partager la même collection
- Arc = Atomic Reference Counting (partage de propriété)
- Mutex = Exclusion mutuelle (un seul accès à la fois)

### 2. **Trait Objects** (`Box<dyn Pokemon>`)
- Polymorphisme dynamique pour différents types de Pokémon
- Permet stockage hétérogène (Flamby et Aquali dans le même vecteur)

### 3. **Pattern matching** et **Options**
- `if let Some(ref mut combat) = combat_state` - Gestion d'état optionnel
- `Option<CombatState>` - Machine d'état exploration/combat

### 4. **Lifetimes implicites**
- Les références utilisées respectent les durées de vie Rust
- Pas de `'a`, `'b` explicites grâce à l'élision

### 5. **Closure et capture**
- Thread de génération capture `Arc<Mutex<Vec<Potion>>>` via `move`

---

## 📊 Répartition du travail

### Phase 1 : Architecture et modules de base
- ✅ Création de `player.rs` - Mouvement et animation du joueur
- ✅ Création de `pokemon.rs` - Trait Pokémon et implémentations
- ✅ Création de `dresseur.rs` - Données du personnage
- ✅ Création de `graphics.rs` - Utilitaires de rendu

### Phase 2 : Système de potion (threads + mutex)
- ✅ Création de `potion.rs` - Structure Potion
- ✅ Création de `potion_manager.rs` - **Thread de génération + Mutex**
- ✅ Intégration dans `main.rs` - Rendu et collisions

### Phase 3 : Combat et rencontres
- ✅ Création de `combat.rs` - Système de combat complet
- ✅ Intégration de Célèbi avec détection de collision
- ✅ Machine d'état exploration ↔ combat

### Phase 4 : Finalisation
- ✅ Création de `inventory.rs` - Inventaire du joueur
- ✅ Création de `trainer_animations.rs` - Animations du dresseur
- ✅ Pop-up de victoire quand Célèbi est vaincu
- ✅ Arrêt du jeu après victoire

---

## 🚀 Comment compiler et exécuter

```bash
# Compiler le projet
cargo build

# Exécuter le jeu
cargo run

# Compiler en mode release (optimisé)
cargo build --release
```

### Contrôles du jeu
- **Flèches** : Se déplacer
- **Entrée** : Interagir avec pop-ups (rencontre, victoire)
- **Échap** : Annuler une pop-up de rencontre
- **1/2/3/4** : Actions en combat (attaque, pokéball, potion, fuite)
- **E** : Quitter le jeu

---

## 📋 Validation du CDC (Cahier des Charges)

| Critère | Statut | Détail |
|---------|--------|--------|
| **Threads** | ✅ Satisfait | 1 thread de génération de potions dans `potion_manager.rs` |
| **Mutex** | ✅ Satisfait | Arc<Mutex<Vec<Potion>>> pour synchronisation thread-safe |
| **Concurrence** | ✅ Satisfait | Thread génère potions toutes les 2s en parallèle du gameplay |
| **Modularité** | ✅ Satisfait | 12 modules séparés avec responsabilités claires |
| **Gameplay** | ✅ Satisfait | Exploration, rencontres, combat contre Célèbi |
| **Victoire** | ✅ Satisfait | Pop-up de victoire + arrêt du jeu après défaite de Célèbi |

---

## 🔮 Améliorations futures possibles

- [ ] Multiples rencontres Pokémon sur la carte
- [ ] Système de leveling et d'expérience
- [ ] Mécanique réelle de capture de Pokémon
- [ ] Guérison des Pokémon avec les potions
- [ ] Dialogues et quêtes
- [ ] Sauvegarde du progrès
- [ ] Musique et effets sonores
- [ ] Multiplayer local ou réseau

---

## 👨‍💻 Auteur

Développé par Matthieu comme projet d'évaluation ESEO (Cycles Ingénieur, Module Rust)

**Date** : Novembre 2025  
**Licence** : Voir le fichier `LICENSE`


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