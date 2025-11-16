use macroquad::prelude::*;

use pokemon_lite::player::Player;
use pokemon_lite::graphics::draw_ui;
use pokemon_lite::pokedex::load_pokemon_sprites;
use pokemon_lite::pokemon::{Pokemon, Flamby};
use pokemon_lite::trainer_animations::TrainerAnimations;
use pokemon_lite::potion_manager::PotionManager;
use pokemon_lite::pokemon_spawner::PokemonSpawner;

#[macroquad::main("Pokemon Lite")]
async fn main() {
    let background_texture = load_texture("texture/Game Boy Advance - Pokemon Mystery Dungeon_ Red Rescue Team - Backgrounds - Pokemon Square.png")
        .await
        .expect("Erreur: Impossible de charger l'image de fond");
    
    let trainer_animations = TrainerAnimations::load()
        .await
        .expect("Erreur: Impossible de charger les animations du dresseur");
    
    let pokemon_sprites = load_pokemon_sprites()
        .expect("Erreur: Impossible de charger les sprites Pokémon");

    let pokemon_textures: Vec<Texture2D> = pokemon_sprites
        .iter()
        .map(|s| s.sprite.to_macroquad_texture())
        .collect();
    
    // Créer le joueur au démarrage au milieu-gauche de la map
    // Position: x=200 pixels, y=400 pixels
    let mut joueur = Player::new("Sacha".to_string(), 200, 320);

    // Créer le gestionnaire de potions avec le thread générateur
    let potion_manager = PotionManager::new();
    
    // Inventaire du joueur (nombre de potions)
    let mut inventaire_potions = 0;

    // Créer le spawner de Pokémon sur la map
    let mut pokemon_spawner = PokemonSpawner::new();

    // Position du Célèbi sur la map (centre)
    let celebi_x = 400;
    let celebi_y = 330;

    // État du combat / pop-up
    let mut show_encounter_popup = false;
    let mut encounter_pokemon_id: Option<u32> = None;

    // ========== BOUCLE DE JEU PRINCIPALE ==========
    loop {
        // === ÉTAPE 1 : RÉCUPÉRER LES INPUTS ===
        if is_key_pressed(KeyCode::E) {
            break; // Quitter
        }

        // Déplacement haut (Flèche haut)
        if is_key_down(KeyCode::Up) && joueur.can_move() {
            joueur.move_up();
        }

        // Déplacement bas (Flèche bas)
        if is_key_down(KeyCode::Down) && joueur.can_move() {
            joueur.move_down(1007); // Limite de l'image
        }

        // Déplacement gauche (Flèche gauche)
        if is_key_down(KeyCode::Left) && joueur.can_move() {
            joueur.move_left();
        }

        // Déplacement droite (Flèche droite)
        if is_key_down(KeyCode::Right) && joueur.can_move() {
            joueur.move_right(1064); // Limite de l'image
        }

        // === ÉTAPE 2 : METTRE À JOUR ===
        // Mettre à jour l'animation du dresseur
        let delta_time = get_frame_time();
        joueur.update_animation(delta_time);
        joueur.update_movement_cooldown(delta_time);

        // Vérifier les collisions avec les potions
        let collected = potion_manager.collect_potions_at_position(joueur.x, joueur.y);
        inventaire_potions += collected.len();
        if !collected.is_empty() {
            println!("🎒 Inventaire: {} potion(s)", inventaire_potions);
        }

        // Vérifier les collisions avec les Pokémon ennemis
        let collisions = pokemon_spawner.check_collisions(joueur.x, joueur.y);
        if !collisions.is_empty() {
            println!("⚔️  Combat engagé! Pokémon trouvé!");
            // TODO: Lancer le combat avec le premier Pokémon en collision
            // Pour l'instant, on affiche juste le message
        }

        // Vérifier la collision avec Célèbi
        let dx_celebi = (celebi_x - joueur.x).abs();
        let dy_celebi = (celebi_y - joueur.y).abs();
        if dx_celebi < 25 && dy_celebi < 25 && !show_encounter_popup {
            show_encounter_popup = true;
            encounter_pokemon_id = Some(999); // ID spécial pour Célèbi
            println!("✨ Célèbi détecté! Appuyez sur Entrée pour combattre!");
        }

        // Si la pop-up est active et on appuie sur Entrée
        if show_encounter_popup && is_key_pressed(KeyCode::Enter) {
            println!("⚔️  Combat lancé contre Célèbi!");
            show_encounter_popup = false;
            // TODO: Lancer le combat avec Célèbi
        }

        // Si on appuie sur Échap, fermer la pop-up
        if show_encounter_popup && is_key_pressed(KeyCode::Escape) {
            show_encounter_popup = false;
        }

        // === ÉTAPE 3 : DESSINER ===
        clear_background(BLACK);

        // Afficher l'image de fond
        // texture, position x, position y, couleur (WHITE = pas de tint)
        draw_texture(&background_texture, 0.0, 0.0, WHITE);

        // Afficher les potions sur la map
        let potions = potion_manager.get_potions();
        for potion in potions {
            // Dessiner un cercle rouge ✨ pour chaque potion
            draw_circle(potion.x as f32, potion.y as f32, 8.0, Color::new(1.0, 0.0, 0.0, 0.9));
            // Ajouter une petite décoration (anneau jaune)
            draw_circle_lines(potion.x as f32, potion.y as f32, 12.0, 2.0, Color::new(1.0, 1.0, 0.0, 0.7));
        }

        // Afficher les Pokémon ennemis sur la map
        for encounter in &pokemon_spawner.encounters {
            // Dessiner un cercle bleu 🎮 pour chaque Pokémon ennemi
            draw_circle(encounter.x as f32, encounter.y as f32, 10.0, Color::new(0.0, 0.5, 1.0, 0.9));
            // Ajouter une petite décoration (anneau rouge)
            draw_circle_lines(encounter.x as f32, encounter.y as f32, 15.0, 2.0, Color::new(1.0, 0.0, 0.0, 0.7));
            // Afficher le nom du Pokémon
            draw_text(
                encounter.pokemon.get_nom(),
                encounter.x as f32 - 30.0,
                encounter.y as f32 - 20.0,
                14.0,
                WHITE,
            );
        }

        // Afficher le Célèbi au centre (détectable par collision)
        draw_circle(celebi_x as f32, celebi_y as f32, 12.0, Color::new(0.2, 1.0, 0.8, 0.9));
        draw_circle_lines(celebi_x as f32, celebi_y as f32, 18.0, 3.0, Color::new(1.0, 1.0, 0.0, 0.8));
        draw_text("Celebi", celebi_x as f32 - 25.0, celebi_y as f32 - 25.0, 14.0, WHITE);

        // Afficher le dresseur avec la frame d'animation actuelle
        let frame_name = joueur.get_frame_name();
        if let Some(texture) = trainer_animations.get_frame(&frame_name) {
            let params = DrawTextureParams {
                dest_size: Some(Vec2::new(16.0, 16.0)),
                ..Default::default()
            };
            draw_texture_ex(texture, joueur.x as f32, joueur.y as f32, WHITE, params);
        }

        // Dessiner l'UI
        let (tile_x, tile_y) = joueur.get_tile_position();
        draw_ui(&joueur.nom, joueur.x, joueur.y, tile_x, tile_y);
        
        // Afficher l'inventaire de potions
        draw_text(
            &format!("🧪 Potions: {}", inventaire_potions),
            10.0,
            30.0,
            20.0,
            YELLOW,
        );

        // Afficher la pop-up de rencontre avec Célèbi
        if show_encounter_popup {
            let popup_x = screen_width() / 2.0 - 200.0;
            let popup_y = screen_height() / 2.0 - 100.0;
            let popup_width = 400.0;
            let popup_height = 200.0;

            // Fond semi-transparent noir
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.5));

            // Fenêtre pop-up
            draw_rectangle(popup_x, popup_y, popup_width, popup_height, Color::new(0.2, 0.6, 1.0, 1.0));
            draw_rectangle_lines(popup_x, popup_y, popup_width, popup_height, 3.0, YELLOW);

            // Texte
            draw_text("✨ Rencontre avec Célèbi!", popup_x + 50.0, popup_y + 40.0, 20.0, WHITE);
            draw_text("Appuyez sur ENTRÉE pour combattre", popup_x + 30.0, popup_y + 90.0, 16.0, WHITE);
            draw_text("ou ÉCHAP pour ignorer", popup_x + 60.0, popup_y + 130.0, 16.0, WHITE);
        }

        // === ÉTAPE 4 : AFFICHER ET SYNC FPS ===
        next_frame().await;
    }
}
