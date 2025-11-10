use macroquad::prelude::*;

mod pokemon;
mod combat;
mod dresseur;
mod map;
mod player;
mod graphics;

use map::{Map, TerrainType, TILE_SIZE};
use player::Player;
use graphics::{draw_tile, draw_player, draw_ui};

#[macroquad::main("Pokemon Lite")]
async fn main() {
    // Configuration de la fenêtre (Macroquad gère automatiquement)
    let map_largeur = 15;
    let map_hauteur = 12;
    
    // Créer une map de 15x12 (en tiles)
    let mut map = Map::new(map_largeur as usize, map_hauteur as usize);

    // Placer des hautes herbes (zones sauvages)
    map.set_terrain(5, 3, TerrainType::HauteHerbe);
    map.set_terrain(6, 3, TerrainType::HauteHerbe);
    map.set_terrain(5, 4, TerrainType::HauteHerbe);
    map.set_terrain(6, 4, TerrainType::HauteHerbe);

    // Placer des bâtiments
    map.set_terrain(1, 1, TerrainType::Pharmacie);
    map.set_terrain(12, 10, TerrainType::Batiment1);
    map.set_terrain(13, 10, TerrainType::Batiment2);

    // Placer de l'eau
    for x in 2..5 {
        map.set_terrain(x, 7, TerrainType::Eau);
        map.set_terrain(x, 8, TerrainType::Eau);
    }

    // Placer un chemin
    for x in 0..15 {
        map.set_terrain(x, 0, TerrainType::Chemin);
    }

    // Créer le joueur au démarrage (position 0,0 en pixels = tile (0,0))
    let mut joueur = Player::new("Sacha".to_string(), 0, 56); // Commence à (0, 1) tile

    // ========== BOUCLE DE JEU PRINCIPALE ==========
    loop {
        // === ÉTAPE 1 : RÉCUPÉRER LES INPUTS ===
        if is_key_pressed(KeyCode::P) {
            break; // Quitter
        }

        // Déplacement haut
        if is_key_pressed(KeyCode::Z) {
            let (tile_x, tile_y) = joueur.get_tile_position();
            let nouvelle_pos = (tile_x * TILE_SIZE, (tile_y - 1) * TILE_SIZE);
            if map.peut_se_deplacer(nouvelle_pos.0, nouvelle_pos.1) {
                joueur.move_up();
            }
        }

        // Déplacement bas
        if is_key_pressed(KeyCode::S) {
            let (tile_x, tile_y) = joueur.get_tile_position();
            let nouvelle_pos = (tile_x * TILE_SIZE, (tile_y + 1) * TILE_SIZE);
            if map.peut_se_deplacer(nouvelle_pos.0, nouvelle_pos.1) {
                joueur.move_down(map.hauteur_pixels());
            }
        }

        // Déplacement gauche
        if is_key_pressed(KeyCode::Q) {
            let (tile_x, tile_y) = joueur.get_tile_position();
            let nouvelle_pos = ((tile_x - 1) * TILE_SIZE, tile_y * TILE_SIZE);
            if map.peut_se_deplacer(nouvelle_pos.0, nouvelle_pos.1) {
                joueur.move_left();
            }
        }

        // Déplacement droite
        if is_key_pressed(KeyCode::D) {
            let (tile_x, tile_y) = joueur.get_tile_position();
            let nouvelle_pos = ((tile_x + 1) * TILE_SIZE, tile_y * TILE_SIZE);
            if map.peut_se_deplacer(nouvelle_pos.0, nouvelle_pos.1) {
                joueur.move_right(map.largeur_pixels());
            }
        }

        // === ÉTAPE 2 : METTRE À JOUR ===
        // (Rien à mettre à jour pour l'instant)

        // === ÉTAPE 3 : DESSINER ===
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0)); // Fond noir

        // Dessiner la map
        for y in 0..map.hauteur() {
            for x in 0..map.largeur() {
                if let Some(tile) = map.get_tile(x, y) {
                    draw_tile(x as i32, y as i32, tile.terrain);
                }
            }
        }

        // Dessiner le joueur
        draw_player(joueur.x, joueur.y);

        // Dessiner l'UI
        let (tile_x, tile_y) = joueur.get_tile_position();
        draw_ui(&joueur.nom, joueur.x, joueur.y, tile_x, tile_y);

        // === ÉTAPE 4 : AFFICHER ET SYNC FPS ===
        next_frame().await;
    }
}
