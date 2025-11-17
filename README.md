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
- fait appel à la fonction load() du trainer_animations() qui permet d'afficher les 8 frames différentes du dresseur : gauche, droite, haut, bas et les arrêts. Et fait l'appel à la frame principale et utilise Macroquad pour charger le fond d'écran de l'image PNG "Game Boy... .png"
- permet le lancement des déplacements du joueur lorsque le combat n'est pas initialisé. 
- puis affiche la popup de combat lorsque le combat est initialisé.
- met à jour ensuite les données du dresseur
- vérifies les collisions entre le dresseur et le pokémon Célébie (rond bleu) et avec les potions (ronds rouges)
- affiche les popups : lancement du combat, map avec caractéristiques du joueur, popup de rencontre, popup de victoire ou défaite

**Dépendances** : 
- `macroquad` - Gestion graphique et input
- Tous les autres modules

### 2. **potion.rs** - Gestion des potions
**Responsabilité** : Définition des données et comportements des potions

**Structures** :
- `Potion { x, y, id, hp_restore }` - Représentation d'une potion sur la carte

**Fonctionnalités** :
- Fonction pour générer une potion (random_position())à des coordonnées aléatoire de la taille de la carte
- Fonction pour détecter la collision entre la potion(is_colliding_with_player(self, coordonner x, coordonnée y)) et les coordonnées du joueurs : utilisé par potions_managers


### 3. **potion_manager.rs** - Gestion thread-safe des potions
**Responsabilité** : Gérer l'accès concurrent aux potions via un thread de génération


**Fonctionnalités** :
- Créé une instance de l'objet PotionManager qui contient l'id et la potion pour le main thread.
- Lance le thread secondaire pour la génération aléatoire de potions.
- Synchronisation entre le comptage des potions qui sont créé en background et la récupération des ids de ces potions au niveau du main.
Les deux variables "potions" et "potions_counter" sont protégés par un mutex.
- appel la gestion de collision pour le moment où le dresseur touche une potion qui popup sur la map pour collecter la potion dans son inventaire.

### 4. **pokemon.rs** - Traits et implémentations Pokémon
**Responsabilité** : Définir le comportement commun des Pokémon

**Trait** `Pokemon` :
Utilisation d'un trait pour la modularité au niveau des pokémons. Cela permettra à l'avenir de pouvoir coder l'efficacité d'attaquer entre les types de pokémons. Actuellement ça permet d'implémenter plusieurs méthodes communes à chaque structure individuel des pokémons.
- attaquer()
- est_vivant()
etc...

Actuellement, il y a 3 pokémons de créé: Florizarre, Aqualie et Flamby qui sont du type Pokemon
Chaque objet Pokemon est ajouté dans un vecteur dynamique de type `Box<dyn Pokemon>`


### 5. **combat.rs** - Système de combat tour par tour
**Responsabilité** : Logique et interface du combat

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

**Nouveauté (1)- Gestion d'erreur avec Some()**

Dans les méthodes de la structure CombatState, j'utilise une première vérification pour savoir si les 2 pokémons sont toujours vivants (Some). Car dans le cas où ils sont morts (None) alors la partie est terminée.
Pour modifier l'origine de chaque pokémon j'utilise as_ref() comme méthode pour travailler sur mes Vecteur, ce qui me permet de récupérer la référence de l'objet Pokemon concerné lorsqu'un pokémon inflige des dégâts. 'as_ref()' convertit la structure initiale Option<Box<dyn Pokemon>> en &Box<dyn Pokemon> ce qui me permet d'accéder aux références des méthodes de l'instance.

```
let sauvage_nom: String = self.pokemon_sauvage.as_ref().unwrap().get_nom().clone();
```
Ici ça me permet de récupérer la référence de la variable nom via la méthode get_nom() de l'objet pokemon_sauvage.

**Nouveauté (2)- Rendering graphique avec Macroquad**
Pour l'interface de combat, j'ai utilisé la librairie Macroquad qui permet de faire des dessins. Elle permet notamment d'accéder aux fonctions : 
- draw_rectangle qui prend 5 arguments (position intiale x, y, largeur, hauteur, couleur)
- draw_text(contenu à afficher, position x, position y, taille du texte, couleur)
- draw_rectangle_lines(...) pour faire les bordures

J'ai utilisé ces 3 fonctions pour faire aussi le dessin des boutons pour les 4 actions : attaquer, pokéball, potion, fuir.

### 6. **inventory.rs** - Gestion de l'inventaire du joueur
**Responsabilité** : Stockage et gestion des Pokémon du joueur

**Fonctionnalités** :
- `add_pokemon()` - Ajoute un Pokémon à l'équipe
- `get_current_pokemon()` - Retourne le Pokémon actif

Ce module doit être améliorer pour faire le lien entre l'inventaire du dresseur et le combat. Car actuellement je simule le combat avec un faux ajout du pokemon Flamby dans l'inventaire du dresseur dès le moment où la rencontre se fait avec Célébi : 

```
if show_encounter_popup && !show_victory_popup && is_key_pressed(KeyCode::Enter) {
    show_encounter_popup = false;
    _in_battle = true;
    
    let pokemon_joueur = Box::new(Flamby::new("Flambino".to_string())) as Box<dyn Pokemon>;

    let mut celebi = Flamby::new("Celebi".to_string());
    celebi.set_pv(120);  // Donner beaucoup plus de PV à Célèbi
    let pokemon_celebi = Box::new(celebi) as Box<dyn Pokemon>;

    combat_state = Some(CombatState::new(pokemon_joueur, pokemon_celebi));
}
```
Ce que montre ce code, c'est le fait que j'ajoute manuellement un Pokemon qui s'appelle Flamby dans l'inventaire du dresseur si le dresseur va au combat avec Célébi et si la touche "Enter" a été appuyé. Je set les PV de Célébi et la variable qui est utilisé dans le main pour détecter s'il y a un combat ou non "combat_state". Je créé une instance avec les deux pokémons qui s'affrontent et lorsqu'on sortira de la condition, le main fera appel à dessiner_interface_combat(un type CombatState) pour afficher la fenêtre de combat.


Actuellement on peut ajouter des pokémons dans l'inventaire du dresseur, mais seulement manuellement en hardcode et non avec l'apparition de pokémon sauvage. Et on n'a pas le possibilité de récupérer le pokémon de l'inventaire pour l'envoyer au combat.

```
let mut inventory = Inventory::new();
inventory.add_pokemon(Box::new(Flamby::new("Flambino".to_string())));
inventory.add_pokemon(Box::new(Aquali::new("Aquali".to_string())));
```


### 7. **player.rs** - Gestion du dresseur
**Responsabilité** : État et mouvement du joueur

**Fonctionnalités** :
- `move_up/down/left/right()` - Déplacement avec cooldown (30ms entre mouvements)
- `update_animation()` - Cycle d'animation du sprite (4 frames)
- `can_move()` - Vérification du cooldown

J'utilise Macroquad ici aussi pour faire la détection des touches du clavier pour le déplacement du dresseur. J'utilise libs.rs de Macroquad avec la fonction "get_context()" qui renvoie le contexte générale de mon jeu (clavier, souris, écran, son) et je détecte la touche avec la fontion contains() de la librairie Rust.

### 8. **trainer_animations.rs** - Gestion des animations du dresseur
**Responsabilité** : Charger et gérer les sprites du dresseur

**Fonctionnalités** :
- `get_frame()` - Retourne la texture correspondant à l'animation actuelle
- Support de 4 directions × 4 frames chacune

Chargement asynchrone qui charge les sprites du dresseur. Comme il y a 12 sprites différentes, cela permet d'actualiser les frames en fonction du mouvement du joueur.

### 9. **graphics.rs** - Fonctions utilitaires graphiques
**Responsabilité** : Rendu UI (texte, boîtes, inventaire)

Ce module affiche la fenêtre principale du jeu. Il dessine la fenêtre, le texte à l'intérieur 
Dans ce module, j'ai utilisé timer.rs de la librairie macroquad qui permet de récupérer le fps du jeu. J'ai aussi fait l'affichage des caractéristiques du joueur ainsi que des commandes de contrôles.

**Fonctionnalités** :
- `draw_ui()` - Affiche info joueur (nom, position)

### 10. **lib.rs** - Déclaration des modules
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


### 2. ****[Rand](https://docs.rs/rand/latest/rand/)**** (v0.9)
**Utilisation** : Génération de nombres aléatoires

**Fonctionnalités utilisées** :
- `rand::random_range()` - Génération d'entiers aléatoires dans une plage
- Positions aléatoires des potions (50-1000 en X, 50-950 en Y)

**Note** : Initialement utilisait des APIs dépréciées (`thread_rng()`, `gen_range()`), maintenant corrigé avec `random_range()` moderne

**Utilisé dans** : `potion.rs`, `pokemon.rs`


## 🏗️ Architecture

### Structure des modules

```
pokemon_lite/
├── src/
│   ├── main.rs                 # Boucle de jeu principale
│   ├── player.rs               # Système de mouvement du joueur
│   ├── pokemon.rs              # Logique backend des Pokémon (trait + implémentations)
│   ├── combat.rs               # Système de combat tour par tour
│   ├── potion.rs               # Gestion des potions
│   ├── potion_manager.rs       # Génération thread-safe des potions (Arc<Mutex>)
│   ├── inventory.rs            # Gestion de l'inventaire du joueur
│   ├── trainer_animations.rs   # Gestion des animations du dresseur
│   ├── graphics.rs             # Fonctions graphiques (UI)
│   └── lib.rs                  # Exports des modules
└── texture/
    ├── pokemon/                # Sprites des Pokémon (32×32px)
    ├── dresseur/               # Animations du dresseur (16×24px)
    └── Game Boy Advance - Pokemon Mystery Dungeon_ Red Rescue Team - Backgrounds - Pokemon Square.png     # Map de fond (1064×1007px)
```

---

## Améliorations futures possibles

- [ ] Actions de fuir et attraper un pokémon en faisant les actions : Fuir et Pokéball
- [ ] Ajouter le système d'efficacité pour les types de pokémons
- [ ] Création du rendering des personnages : dresseur, pokéball, potions, pokémons
- [ ] Rencontre de pokémon sauvage sur la carte
- [ ] Système de leveling et d'expérience
- [ ] Mécanique réelle de capture de Pokémon
- [ ] Dialogues et quêtes
- [ ] Sauvegarde du progrès
- [ ] Musique et effets sonores
