use macroquad::prelude::*;
use crate::map::{TerrainType, TILE_SIZE};

// Couleurs pour chaque type de terrain
pub fn get_terrain_color(terrain: TerrainType) -> Color {
    match terrain {
        TerrainType::Herbe => Color::new(0.2, 0.8, 0.2, 1.0),      // Vert clair
        TerrainType::HauteHerbe => Color::new(0.1, 0.6, 0.1, 1.0),  // Vert foncé
        TerrainType::Eau => Color::new(0.2, 0.5, 0.9, 1.0),         // Bleu
        TerrainType::Chemin => Color::new(0.8, 0.7, 0.4, 1.0),      // Beige/marron
        TerrainType::Pharmacie => Color::new(0.9, 0.2, 0.2, 1.0),   // Rouge
        TerrainType::Batiment1 => Color::new(0.7, 0.5, 0.3, 1.0),   // Marron
        TerrainType::Batiment2 => Color::new(0.6, 0.4, 0.2, 1.0),   // Marron foncé
        TerrainType::Dresseur => Color::new(0.8, 0.8, 0.0, 1.0),    // Jaune
    }
}

// Dessiner une tile
pub fn draw_tile(x: i32, y: i32, terrain: TerrainType) {
    let color = get_terrain_color(terrain);
    let screen_x = x as f32 * TILE_SIZE as f32;
    let screen_y = y as f32 * TILE_SIZE as f32;
    
    draw_rectangle(
        screen_x,
        screen_y,
        TILE_SIZE as f32,
        TILE_SIZE as f32,
        color,
    );
    
    // Ajouter une bordure pour bien voir les tiles
    draw_rectangle_lines(
        screen_x,
        screen_y,
        TILE_SIZE as f32,
        TILE_SIZE as f32,
        1.0,
        Color::new(0.3, 0.3, 0.3, 1.0),
    );
    
    // Ajouter un symbole texte pour certains terrains
    let symbole = match terrain {
        TerrainType::Eau => "~",
        TerrainType::HauteHerbe => "#",
        TerrainType::Chemin => "-",
        TerrainType::Pharmacie => "P",
        TerrainType::Batiment1 => "B",
        TerrainType::Batiment2 => "C",
        TerrainType::Dresseur => "D",
        _ => "",
    };
    
    if !symbole.is_empty() {
        draw_text(
            symbole,
            screen_x + TILE_SIZE as f32 / 2.0 - 4.0,
            screen_y + TILE_SIZE as f32 / 2.0 + 5.0,
            20.0,
            BLACK,
        );
    }
}

// Dessiner le joueur (petit carré au centre de sa tile)
pub fn draw_player(joueur_x: i32, joueur_y: i32) {
    let screen_x = joueur_x as f32 + TILE_SIZE as f32 / 4.0;
    let screen_y = joueur_y as f32 + TILE_SIZE as f32 / 4.0;
    let player_size = TILE_SIZE as f32 / 2.0;
    
    draw_rectangle(
        screen_x,
        screen_y,
        player_size,
        player_size,
        Color::new(0.2, 0.2, 0.9, 1.0), // Bleu pour le joueur
    );
    
    // Ajouter une bordure
    draw_rectangle_lines(
        screen_x,
        screen_y,
        player_size,
        player_size,
        2.0,
        BLACK,
    );
}

// Dessiner l'UI (infos du joueur)
pub fn draw_ui(joueur_nom: &str, joueur_x: i32, joueur_y: i32, tile_x: i32, tile_y: i32) {
    let ui_height = 60.0;
    
    // Fond de l'UI
    draw_rectangle(
        0.0,
        screen_height() - ui_height,
        screen_width(),
        ui_height,
        Color::new(0.2, 0.2, 0.2, 0.9),
    );
    
    // Texte d'info
    let info_text = format!(
        "{} - Position: ({}, {}) pixels | ({}, {}) tile | FPS: {:.0}",
        joueur_nom,
        joueur_x,
        joueur_y,
        tile_x,
        tile_y,
        get_fps()
    );
    
    draw_text(
        &info_text,
        10.0,
        screen_height() - 15.0,
        16.0,
        Color::new(0.0, 1.0, 0.0, 1.0), // Vert
    );
    
    // Commandes
    draw_text(
        "Z/S/Q/D: Move | P: Quit",
        10.0,
        screen_height() - 35.0,
        14.0,
        Color::new(1.0, 1.0, 1.0, 1.0), // Blanc
    );
}
