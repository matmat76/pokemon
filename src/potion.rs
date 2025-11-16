use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Potion {
    pub x: i32,
    pub y: i32,
    pub id: u32,
    pub hp_restore: i32, // Nombre de PV restaurés
}

impl Potion {
    pub fn new(x: i32, y: i32, id: u32) -> Self {
        Potion {
            x,
            y,
            id,
            hp_restore: 20, // Une potion restaure 20 PV par défaut
        }
    }

    /// Génère une position aléatoire valide sur la map
    /// Map : 1064px de largeur, 1007px de hauteur
    pub fn random_position(id: u32) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Génère une position aléatoire (avec des marges pour pas être trop près des bords)
        let x = rng.gen_range(50..1000);
        let y = rng.gen_range(50..950);
        
        Potion::new(x, y, id)
    }

    /// Vérifie si le joueur (position x, y) touche la potion
    /// Distance de collision : 20 pixels
    pub fn is_colliding_with_player(&self, player_x: i32, player_y: i32) -> bool {
        let dx = (self.x - player_x).abs();
        let dy = (self.y - player_y).abs();
        let collision_distance = 20;
        
        dx < collision_distance && dy < collision_distance
    }
}
