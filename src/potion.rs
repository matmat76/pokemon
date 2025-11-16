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

    pub fn random_position(id: u32) -> Self {
        let x = rand::random_range(50..1000);
        let y = rand::random_range(50..950);
        
        Potion::new(x, y, id)
    }

    pub fn is_colliding_with_player(&self, player_x: i32, player_y: i32) -> bool {
        let dx = (self.x - player_x).abs();
        let dy = (self.y - player_y).abs();
        let collision_distance = 20;
        
        dx < collision_distance && dy < collision_distance
    }
}
