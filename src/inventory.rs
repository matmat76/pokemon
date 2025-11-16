use crate::pokemon::Pokemon;

pub struct Inventory {
    pub pokemons: Vec<Box<dyn Pokemon>>,
    pub current_pokemon_index: usize,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            pokemons: Vec::new(),
            current_pokemon_index: 0,
        }
    }

    /// Ajoute un Pokémon à l'inventaire
    pub fn add_pokemon(&mut self, pokemon: Box<dyn Pokemon>) {
        self.pokemons.push(pokemon);
    }

    /// Retourne le Pokémon actuellement sélectionné
    pub fn get_current_pokemon(&self) -> Option<&Box<dyn Pokemon>> {
        self.pokemons.get(self.current_pokemon_index)
    }

    /// Retourne une copie du Pokémon actuellement sélectionné (pour le combat)
    /// Attention: On clone pas le trait object, on crée une nouvelle instance
    pub fn get_current_pokemon_for_combat(&self) -> Option<Box<dyn Pokemon>> {
        if let Some(_pokemon) = self.get_current_pokemon() {
            // TODO: Implémenter le clonage des Pokémon ou créer une nouvelle instance
            // Pour l'instant, on retourne None
            None
        } else {
            None
        }
    }

    /// Change le Pokémon actif (Flèches gauche/droite)
    pub fn switch_to_previous(&mut self) {
        if self.pokemons.len() > 0 {
            if self.current_pokemon_index == 0 {
                self.current_pokemon_index = self.pokemons.len() - 1;
            } else {
                self.current_pokemon_index -= 1;
            }
        }
    }

    pub fn switch_to_next(&mut self) {
        if self.pokemons.len() > 0 {
            self.current_pokemon_index = (self.current_pokemon_index + 1) % self.pokemons.len();
        }
    }

    /// Affiche l'inventaire des Pokémon
    pub fn display_inventory(&self) {
        println!("\n=== Inventaire Pokémon ===");
        for (i, pokemon) in self.pokemons.iter().enumerate() {
            let marker = if i == self.current_pokemon_index { "→" } else { " " };
            println!("{} [{}] {}", marker, i + 1, pokemon.get_nom());
        }
    }
}
