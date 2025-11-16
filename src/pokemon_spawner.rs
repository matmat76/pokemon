use crate::pokemon::{Pokemon, Flamby, Aquali};

pub struct PokemonEncounter {
    pub pokemon: Box<dyn Pokemon>,
    pub x: i32,
    pub y: i32,
    pub id: u32,
}

impl PokemonEncounter {
    pub fn new(pokemon: Box<dyn Pokemon>, x: i32, y: i32, id: u32) -> Self {
        PokemonEncounter {
            pokemon,
            x,
            y,
            id,
        }
    }

    /// Vérifie si le joueur (position x, y) touche ce Pokémon
    /// Distance de collision : 25 pixels
    pub fn is_colliding_with_player(&self, player_x: i32, player_y: i32) -> bool {
        let dx = (self.x - player_x).abs();
        let dy = (self.y - player_y).abs();
        let collision_distance = 25;
        
        dx < collision_distance && dy < collision_distance
    }
}

pub struct PokemonSpawner {
    pub encounters: Vec<PokemonEncounter>,
}

impl PokemonSpawner {
    pub fn new() -> Self {
        let mut spawner = PokemonSpawner {
            encounters: Vec::new(),
        };
        
        // Ajouter quelques Pokémon sur la map
        spawner.add_pokemon(
            Box::new(Aquali::new("Aquali".to_string())),
            600,
            300,
        );
        
        spawner.add_pokemon(
            Box::new(Flamby::new("Flamby2".to_string())),
            300,
            500,
        );
        
        spawner
    }

    /// Ajoute un Pokémon à la map
    pub fn add_pokemon(&mut self, pokemon: Box<dyn Pokemon>, x: i32, y: i32) {
        let id = self.encounters.len() as u32;
        let encounter = PokemonEncounter::new(pokemon, x, y, id);
        self.encounters.push(encounter);
        println!("🎮 Pokémon {} ajouté à ({}, {})", id, x, y);
    }

    /// Vérifie les collisions avec le joueur et retourne les IDs des Pokémon en collision
    pub fn check_collisions(&self, player_x: i32, player_y: i32) -> Vec<u32> {
        self.encounters
            .iter()
            .filter(|e| e.is_colliding_with_player(player_x, player_y))
            .map(|e| e.id)
            .collect()
    }

    /// Supprime un Pokémon après le combat
    pub fn remove_pokemon(&mut self, pokemon_id: u32) {
        self.encounters.retain(|e| e.id != pokemon_id);
        println!("☠️  Pokémon {} vaincu et supprimé!", pokemon_id);
    }
}
