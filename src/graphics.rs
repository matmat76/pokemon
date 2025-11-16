/**
 * Fonctions graphiques pour le rendu de l'UI
 */
use macroquad::prelude::*;

/// Dessiner l'UI (infos du joueur en bas de l'écran)
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
        "Flèches: Move | E: Quit",
        10.0,
        screen_height() - 35.0,
        14.0,
        Color::new(1.0, 1.0, 1.0, 1.0), // Blanc
    );
}
