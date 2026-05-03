# Pokemon Lite

Un jeu Pokémon simplifié développé en **Rust** dans le cadre du projet d'évaluation ESEO (Cycle Ingénieur I3 — Module Rust).

---

## Table des matières

- [Aperçu du jeu](#aperçu-du-jeu)
- [Fonctionnalités](#fonctionnalités)
- [Prérequis](#prérequis)
- [Installation et lancement](#installation-et-lancement)
- [Contrôles](#contrôles)
- [Mécanique de jeu](#mécanique-de-jeu)
- [Architecture du projet](#architecture-du-projet)
- [Concepts Rust utilisés](#concepts-rust-utilisés)
- [Améliorations futures](#améliorations-futures)
- [Auteur](#auteur)

---

## Aperçu du jeu

**Pokemon Lite** est un jeu 2D d'exploration et de combat tour par tour. Le joueur incarne un dresseur qui se déplace sur une carte inspirée de Pokémon Mystery Dungeon. L'objectif est de se rendre à l'emplacement de **Célébi** (symbolisé par un rond bleu sur la carte), de déclencher la rencontre et de le battre en combat.

En chemin, le dresseur peut collecter des **potions** qui apparaissent aléatoirement sur la carte, générées en arrière-plan par un thread dédié.

---

## Fonctionnalités

- **Exploration** d'une carte 2D avec détection de collisions
- **Animations du dresseur** : 12 sprites couvrant 4 directions (haut, bas, gauche, droite) × 3 états (arrêt, course gauche, course droite)
- **Génération asynchrone de potions** toutes les 2 secondes via un thread secondaire (`Arc<Mutex<T>>`)
- **Collecte de potions** automatique par collision avec le dresseur
- **Système de combat tour par tour** : attaque, pokéball, potion, fuite
- **Interface de combat** : barres de HP, log de combat, 4 boutons d'action
- **Rencontre avec Célébi** déclenchée par collision (120 PV, plus résistant que les Pokémon normaux)

---

## Prérequis

- **Rust** (version stable récente) — [installer via rustup](https://rustup.rs/)
- **Cargo** (inclus avec Rust)
- Sur **Linux** uniquement : quelques dépendances système pour macroquad :
  ```bash
  sudo apt install libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
  ```
  Sur **macOS** et **Windows**, aucune dépendance supplémentaire.

Vérifier l'installation :
```bash
rustc --version
cargo --version
```

---

## Installation et lancement

```bash
# 1. Cloner le dépôt
git clone git@github.com:matmat76/pokemon.git
cd pokemon

# 2. Compiler et lancer le jeu
cargo run
```

La première compilation peut prendre une minute (téléchargement des dépendances). Les lancements suivants sont beaucoup plus rapides.

Pour une version optimisée (plus fluide) :
```bash
cargo run --release
```

---

## Contrôles

| Touche | Action |
|--------|--------|
| `↑ ↓ ← →` | Déplacer le dresseur |
| `E` | Quitter le jeu |
| `Entrée` | Confirmer / Lancer le combat lors d'une rencontre |

**En combat :**

| Touche | Action |
|--------|--------|
| `1` ou `A` | Attaquer |
| `2` ou `B` | Lancer une Pokéball (non implémenté) |
| `3` ou `C` | Utiliser une potion (+30 PV, si inventaire > 0) |
| `4` ou `D` | Fuir (non implémenté) |

---

## Mécanique de jeu

### Exploration

Le joueur se déplace pixel par pixel sur la carte (1064×1007 px). Un cooldown de 10 ms entre chaque déplacement assure un mouvement fluide. Les bordures de la carte bloquent le joueur.

Deux types d'objets sont présents sur la carte :
- **Célébi** (rond bleu, position fixe à ~400, 330) : déclenche une rencontre au contact
- **Potions** (ronds rouges) : collectées automatiquement au contact, ajoutées à l'inventaire

### Génération des potions

Un thread secondaire génère une nouvelle potion toutes les 2 secondes à une position aléatoire (X: 50–1000, Y: 50–950). La liste des potions est partagée entre le thread de génération et le thread principal via `Arc<Mutex<Vec<Potion>>>`.

### Rencontre et combat

Lorsque le dresseur touche Célébi, une popup d'information s'affiche. En appuyant sur `Entrée`, le combat se lance.

Le combat oppose **Flambino** (un Flamby à 50 PV, 15 de dégâts) à **Célébi** (un Flamby avec 120 PV).

**Déroulement d'un tour :**
1. Le joueur choisit une action (touches 1–4 ou A–D)
2. Le Pokémon du joueur agit immédiatement
3. Attente de 2 secondes
4. Célébi attaque en retour (si encore vivant)
5. Attente de 2 secondes
6. Le tour recommence

**Fin du combat :**
- Si les PV de Célébi tombent à 0 → **Victoire** (popup affichée)
- Si les PV de Flambino tombent à 0 → **Défaite** (popup affichée)

### Pokémon disponibles

| Pokémon | PV max | Dégâts par attaque |
|---------|--------|--------------------|
| Flamby  | 50     | 15                 |
| Aquali  | 70     | 10                 |
| Florizarre | 60  | 12                 |

Les trois Pokémon partagent le même trait `Pokemon` mais ont des statistiques différentes.

---

## Architecture du projet

```
pokemon_lite/
├── src/
│   ├── main.rs               # Boucle de jeu, état global, orchestration
│   ├── lib.rs                # Déclarations des modules
│   ├── player.rs             # Déplacement, animation et état du dresseur
│   ├── pokemon.rs            # Trait Pokemon + 3 implémentations (Flamby, Aquali, Florizarre)
│   ├── combat.rs             # Logique du combat et rendu de l'interface
│   ├── potion.rs             # Structure Potion, positions aléatoires, collision
│   ├── potion_manager.rs     # Gestion thread-safe des potions (Arc<Mutex>)
│   ├── inventory.rs          # Inventaire du joueur (Pokémon capturés)
│   ├── trainer_animations.rs # Chargement et accès aux 12 sprites du dresseur
│   └── graphics.rs           # Affichage de la barre d'info (position, FPS, commandes)
└── texture/
    ├── dresseur/             # 12 sprites PNG du dresseur (16×24 px chacun)
    └── *.png                 # Fond de carte (1064×1007 px)
```

### Responsabilités des modules

| Module | Rôle |
|--------|------|
| `main.rs` | Boucle principale macroquad, gestion des états (exploration / rencontre / combat / résultat), rendu global |
| `player.rs` | Position du joueur, direction, cooldown de mouvement, calcul de la frame d'animation |
| `pokemon.rs` | Trait polymorphe `Pokemon` avec `attaquer()`, `prendre_degats()`, `est_vivant()`, `get_pv()`, etc. |
| `combat.rs` | `CombatState` avec tour par tour, timer de 2s, traitement des inputs et rendu de la fenêtre de combat |
| `potion.rs` | `Potion { x, y, id, hp_restore }`, génération aléatoire, détection de collision |
| `potion_manager.rs` | Thread secondaire de génération, accès concurrent via `Arc<Mutex<Vec<Potion>>>` |
| `inventory.rs` | Vecteur de `Box<dyn Pokemon>`, ajout et accès au Pokémon actif |
| `trainer_animations.rs` | `HashMap<String, Texture2D>` des 12 frames, chargement async, accès par nom |
| `graphics.rs` | Barre d'info en bas de l'écran : nom, coordonnées pixel, position tile, FPS, commandes |

---

## Concepts Rust utilisés

### Trait objects et polymorphisme

```rust
pub trait Pokemon {
    fn attaquer(&self) -> i32;
    fn prendre_degats(&mut self, degats: i32);
    fn est_vivant(&self) -> bool;
    fn get_pv(&self) -> i32;
    // ...
}
```

Chaque espèce (`Flamby`, `Aquali`, `Florizarre`) implémente ce trait. Les Pokémon sont stockés et passés comme `Box<dyn Pokemon>`, ce qui permet d'écrire du code générique indépendant de l'espèce concrète.

### Concurrence avec `Arc<Mutex<T>>`

```rust
// Dans potion_manager.rs
let potions: Arc<Mutex<Vec<Potion>>> = Arc::new(Mutex::new(Vec::new()));
let potions_clone = Arc::clone(&potions);

thread::spawn(move || {
    loop {
        thread::sleep(Duration::from_secs(2));
        let mut lock = potions_clone.lock().unwrap();
        lock.push(Potion::random_position(/* id */));
    }
});
```

`Arc` permet le partage de propriété entre threads ; `Mutex` garantit l'accès exclusif lors des lectures/écritures.

### `Option<T>` pour la gestion d'état

Les Pokémon en combat sont des `Option<Box<dyn Pokemon>>` : `Some(pokemon)` tant qu'ils sont vivants, `None` une fois KO. Cela évite les pointeurs nuls et force à traiter explicitement le cas "plus de Pokémon".

### Async/await pour le chargement des ressources

```rust
pub async fn load() -> TrainerAnimations {
    let mut frames = HashMap::new();
    for name in FRAME_NAMES {
        let texture = load_texture(&format!("texture/dresseur/{}.png", name)).await.unwrap();
        frames.insert(name.to_string(), texture);
    }
    TrainerAnimations { frames }
}
```

macroquad expose une API asynchrone pour le chargement de textures ; `async/await` permet d'attendre le chargement sans bloquer.

### Enums pour la machine à états

```rust
pub enum ActionCombat { Attaquer, Pokeball, Potion, Fuir, Aucune }
pub enum Direction { Up, Down, Left, Right }
pub enum AnimationState { RunningLeft, RunningRight }
```

Les enums + `match` remplacent les chaînes de conditions `if/else` et garantissent l'exhaustivité à la compilation.

---

## Améliorations futures

- [ ] Implémenter les actions **Fuir** et **Pokéball** dans le combat
- [ ] Système d'efficacité entre types (Feu > Plante > Eau > Feu)
- [ ] Afficher les sprites des Pokémon sur la carte et en combat
- [ ] Rencontres aléatoires avec des Pokémon sauvages pendant l'exploration
- [ ] Lier l'inventaire au combat (envoyer le bon Pokémon depuis l'inventaire)
- [ ] Système d'expérience et de montée de niveau
- [ ] Mécanique de capture réelle avec la Pokéball
- [ ] Dialogues PNJ et quêtes
- [ ] Sauvegarde et chargement de la progression
- [ ] Musique et effets sonores

---

## Auteur

Développé par **Matthieu Tremblay** dans le cadre du projet d'évaluation ESEO I3 — Module Rust.
