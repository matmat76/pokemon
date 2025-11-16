use macroquad::prelude::*;

use pokemon_lite::player::Player;
use pokemon_lite::graphics::draw_ui;
use pokemon_lite::pokedex::load_pokemon_sprites;
use pokemon_lite::pokemon_renderer::PokemonRenderable;
use pokemon_lite::pokemon::{Pokemon, Flamby};
use pokemon_lite::trainer_animations::TrainerAnimations;

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
    let flamby = Box::new(Flamby::new("Flambino".to_string())) as Box<dyn Pokemon>;
    
    // Créer le PokemonRenderable en associant:
    // - Le Pokémon logique (Flamby avec stats)
    // - La texture graphique (Celebi - index 3)
    // - La position sur la map (x=400, y=300) en pixels
    let pokemon_renderable = PokemonRenderable::new(
        flamby,
        pokemon_textures[3].clone(), // Afficher Celebi (index 3)
        400.0,               // x position
        330.0,               // y position
    );
    
    // Créer le joueur au démarrage au milieu-gauche de la map
    // Position: x=200 pixels, y=400 pixels
    let mut joueur = Player::new("Sacha".to_string(), 200, 320);

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

        // === ÉTAPE 3 : DESSINER ===
        clear_background(BLACK);

        // Afficher l'image de fond
        // texture, position x, position y, couleur (WHITE = pas de tint)
        draw_texture(&background_texture, 0.0, 0.0, WHITE);

        // Afficher le dresseur avec la frame d'animation actuelle
        let frame_name = joueur.get_frame_name();
        if let Some(texture) = trainer_animations.get_frame(&frame_name) {
            let params = DrawTextureParams {
                dest_size: Some(Vec2::new(16.0, 16.0)),
                ..Default::default()
            };
            draw_texture_ex(texture, joueur.x as f32, joueur.y as f32, WHITE, params);
        }

        // Afficher le Pokémon renderable!
        // C'est ici qu'on utilise la méthode afficher() qu'on a codée
        pokemon_renderable.afficher();

        // Dessiner l'UI
        let (tile_x, tile_y) = joueur.get_tile_position();
        draw_ui(&joueur.nom, joueur.x, joueur.y, tile_x, tile_y);

        // === ÉTAPE 4 : AFFICHER ET SYNC FPS ===
        next_frame().await;
    }
}
