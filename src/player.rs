// Taille d'une tile en pixels
pub const TILE_SIZE: i32 = 56;

// Représente le joueur sur la map
pub struct Player {
    pub nom: String,
    pub x: i32,  // Position en pixels (pas en indices)
    pub y: i32,  // Position en pixels (pas en indices)
}

impl Player {
    pub fn new(nom: String, x: i32, y: i32) -> Self {
        Player { nom, x, y }
    }

    // Se déplacer vers la droite de TILE_SIZE pixels
    pub fn move_right(&mut self, largeur_max: i32) {
        let nouvelle_x = self.x + TILE_SIZE;
        if nouvelle_x < largeur_max {
            self.x = nouvelle_x;
        }
    }

    // Se déplacer vers la gauche de TILE_SIZE pixels
    pub fn move_left(&mut self) {
        let nouvelle_x = self.x - TILE_SIZE;
        if nouvelle_x >= 0 {
            self.x = nouvelle_x;
        }
    }

    // Se déplacer vers le bas de TILE_SIZE pixels
    pub fn move_down(&mut self, hauteur_max: i32) {
        let nouvelle_y = self.y + TILE_SIZE;
        if nouvelle_y < hauteur_max {
            self.y = nouvelle_y;
        }
    }

    // Se déplacer vers le haut de TILE_SIZE pixels
    pub fn move_up(&mut self) {
        let nouvelle_y = self.y - TILE_SIZE;
        if nouvelle_y >= 0 {
            self.y = nouvelle_y;
        }
    }

    // Afficher la position du joueur (en pixels ET en indices de tile)
    pub fn afficher_position(&self) {
        let tile_x = self.x / TILE_SIZE;
        let tile_y = self.y / TILE_SIZE;
        println!(
            "Position de {} : ({}, {}) pixels | ({}, {}) tile",
            self.nom, self.x, self.y, tile_x, tile_y
        );
    }

    // Obtenir la position en indices de tile (utile pour vérifier les collisions)
    pub fn get_tile_position(&self) -> (i32, i32) {
        (self.x / TILE_SIZE, self.y / TILE_SIZE)
    }
}
