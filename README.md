# Pokémon Lite - Jeu Pokémon en Rust

##  Auteur

Développé par Matthieu comme projet d'évaluation ESEO (Cycles Ingénieur, Module Rust)

## 📋 Vue d'ensemble

**Pokémon Lite** est un jeu Pokémon simplifié développé en **Rust** utilisant la bibliothèque graphique **macroquad** et **image**. Le jeu met en place un système de combat tour par tour avec un dresseur explorant une carte pour rencontrer et combattre Célèbi, le Pokémon légendaire.

### Fonctionnalités principales
- Exploration d'une carte interactive
- Génération asynchrone de potions (thread + mutex)
- Système de combat tour par tour
- Rencontres avec Pokémon ennemis
- Inventaire de potions collectables
- Animations du dresseur

---

## Architecture et Modules

### 1. **main.rs** - Boucle de jeu principale
**Responsabilité** : Orchestration générale du jeu, gestion d'état et boucle graphique

**Fonctionnalités clés** :
- Initialisation du contexte macroquad
- Boucle de jeu (input → update → render)
- Gestion de la machine d'état (exploration ↔ combat)
- Gestion des collisions (joueur-potions, joueur-Pokémon, joueur-Célèbi) : réalisé par l'IA
- Affichage des pop-ups (rencontre, victoire) : réalisé par l'IA 
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

### 10. **sprite_loader.rs** - Chargement et traitement des images
**Responsabilité** : Utilitaires pour charger et manipuler les sprites PNG

**Fonctionnalités** :
- `load_image_file()` - Charge une image PNG et la convertit en RgbaImage
- `crop_sprite()` - Extrait une région rectangulaire d'une image
- `remove_blue_background()` - Rend transparent le fond bleu Pokémon classique
- `rgba_to_macroquad_texture()` - Convertit une RgbaImage en texture Macroquad
- Structure `Sprite` - Wrapper pour stocker et manipuler les sprites

**Dépendances** :
- `image` crate pour le traitement bas niveau des pixels

### 11. **graphics.rs** - Fonctions utilitaires graphiques
**Responsabilité** : Rendu UI (texte, boîtes, inventaire)

**Fonctionnalités** :
- `draw_ui()` - Affiche info joueur (nom, position, tile)
- Rendu du HUD de jeu

### 12. **dresseur.rs** - Données du dresseur
**Responsabilité** : Définition du personnage joueur

**Données** :
- Nom, position (x, y)
- État animation

### 13. **lib.rs** - Déclaration des modules

---

## 🔧 Bibliothèques externes

### 1. ****[Macroquad](https://docs.rs/macroquad/latest/macroquad/)**** (v0.4.14)
**Utilisation** : Framework graphique 2D complet

**Fonctionnalités utilisées** :
- Gestion de la fenêtre et boucle de jeu
- Rendu de textures et formes (cercles, rectangles)
- Système d'input (clavier)
- Chargement d'images PNG
- Gestion de la transparence et couleurs (RGBA)

**Caractéristiques avancées** :
- `DrawTextureParams` - Rendu personnalisé des textures (mise à l'échelle)
- `screen_width()` / `screen_height()` - Détection dynamique de la résolution
- `get_frame_time()` - Delta time pour animations fluides

**Utilisé dans** : `main.rs`, `combat.rs`, `graphics.rs`

### 2. ****[Image](https://docs.rs/image/latest/image/)**** (v0.25)
**Utilisation** : Traitement et manipulation d'images PNG

**Fonctionnalités utilisées** :
- `ImageReader::open()` - Chargement de fichiers PNG
- `DynamicImage::to_rgba8()` - Conversion en format RGBA (4 canaux: R, G, B, Alpha)
- Manipulation directe des pixels (accès R, G, B, A)
- Détection et suppression des fonds colorés (bleu Pokémon classique)
- Extraction de régions d'image (crop de spritesheets)

**Utilisé dans** : `sprite_loader.rs`

### 3. ****[Rand](https://docs.rs/rand/latest/rand/)**** (v0.9)
**Utilisation** : Génération de nombres aléatoires

**Fonctionnalités utilisées** :
- `rand::random_range()` - Génération d'entiers aléatoires dans une plage
- Positions aléatoires des potions (50-1000 en X, 50-950 en Y)
- Positionnement des Pokémon ennemis
- Dégâts aléatoires en combat (entre min et max)

**Note** : Initialement utilisait des APIs dépréciées (`thread_rng()`, `gen_range()`), maintenant corrigé avec `random_range()` moderne

**Utilisé dans** : `potion.rs`, `pokemon.rs`


### Contrôles du jeu
- **Flèches** : Se déplacer
- **Entrée** : Interagir avec pop-ups (rencontre, victoire)
- **Échap** : Annuler une pop-up de rencontre
- **1/2/3/4** : Actions en combat (attaque, pokéball, potion, fuite)
- **E** : Quitter le jeu



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
    ├── pokemon/                # Sprites des Pokémon (32×32px), utilisé partiellement dans le code
    ├── dresseur/               # Animations du dresseur (16×24px)
    └── pokemon_lite/texture/Game Boy Advance - Pokemon Mystery Dungeon_ Red Rescue Team - Backgrounds - Pokemon Square.png     # Map de fond (1064×1007px)
```
Un dresseur possède des pokémons dans un Vecteur dynamique et un Pokemon possèdes les différentes méthodes : attaquer, prendre_degats, est_vivant, etc... pour lancer un combat. 

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
- **Cooldown de mouvement** : 0.01 secondes (10ms)
- **Taille du Pokémon affiché** : 64×64 pixels
- **Taille du dresseur affiché** : 16×16 pixels
- **Taille de la map** : 1064×1007 pixels



## Améliorations futures possibles

- [ ] Rencontre de pokémon sauvage sur la carte : déjà en cours mais non implémentable 
- [ ] Système de leveling et d'expérience
- [ ] Mécanique réelle de capture de Pokémon
- [ ] Dialogues et quêtes
- [ ] Sauvegarde du progrès
- [ ] Musique et effets sonores
