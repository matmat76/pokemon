use crate::pokemon::Pokemon;

pub const TILE_SIZE: i32 = 56; // Même taille que dans player.rs

// Énumération pour les différents types de terrain
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TerrainType {
    Herbe,           // Herbe normale
    HauteHerbe,      // Zone sauvage avec Pokémons
    Eau,             // Lac ou rivière
    Chemin,          // Chemin
    Pharmacie,       // Pharmacie (centre Pokémon)
    Batiment1,       // Bâtiment type 1
    Batiment2,       // Bâtiment type 2
    Dresseur,        // Position d'un dresseur
}

// Représente une case de la map
pub struct Tile {
    pub terrain: TerrainType,
    pub pokémons: Vec<Box<dyn Pokemon>>, // Les Pokémons présents sur cette case
}

impl Tile {
    pub fn new(terrain: TerrainType) -> Self {
        Tile {
            terrain,
            pokémons: Vec::new(),
        }
    }

    // Ajouter un Pokémon à cette case
    pub fn ajouter_pokemon(&mut self, poke: Box<dyn Pokemon>) {
        self.pokémons.push(poke);
    }

    // Retirer un Pokémon (pour les rencontres)
    pub fn retirer_pokemon(&mut self, index: usize) -> Option<Box<dyn Pokemon>> {
        if index < self.pokémons.len() {
            Some(self.pokémons.remove(index))
        } else {
            None
        }
    }
}

// La map complète
pub struct Map {
    largeur: usize,
    hauteur: usize,
    grille: Vec<Vec<Tile>>, // Grille 2D de tiles
}

impl Map {
    // Créer une nouvelle map
    pub fn new(largeur: usize, hauteur: usize) -> Self {
        let mut grille = Vec::new();

        // Créer une grille vide avec de l'herbe
        for _ in 0..hauteur {
            let mut ligne = Vec::new();
            for _ in 0..largeur {
                ligne.push(Tile::new(TerrainType::Herbe));
            }
            grille.push(ligne);
        }

        Map {
            largeur,
            hauteur,
            grille,
        }
    }

    // Obtenir la tile à une position donnée
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < self.largeur && y < self.hauteur {
            Some(&self.grille[y][x])
        } else {
            None
        }
    }

    // Obtenir une référence mutable à une tile
    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        if x < self.largeur && y < self.hauteur {
            Some(&mut self.grille[y][x])
        } else {
            None
        }
    }

    // Changer le type de terrain d'une case
    pub fn set_terrain(&mut self, x: usize, y: usize, terrain: TerrainType) {
        if let Some(tile) = self.get_tile_mut(x, y) {
            tile.terrain = terrain;
        }
    }

    // Afficher la map en console (vue simple)
    pub fn afficher(&self) {
        println!("\n=== MAP ===");
        for y in 0..self.hauteur {
            for x in 0..self.largeur {
                let symbole = match self.grille[y][x].terrain {
                    TerrainType::Herbe => ".",
                    TerrainType::HauteHerbe => "#",
                    TerrainType::Eau => "~",
                    TerrainType::Chemin => "-",
                    TerrainType::Pharmacie => "P",
                    TerrainType::Batiment1 => "B",
                    TerrainType::Batiment2 => "C",
                    TerrainType::Dresseur => "D",
                };
                print!("{} ", symbole);
            }
            println!();
        }
        println!("===========\n");
    }

    pub fn largeur(&self) -> usize {
        self.largeur
    }

    pub fn hauteur(&self) -> usize {
        self.hauteur
    }

    // Obtenir la largeur en pixels
    pub fn largeur_pixels(&self) -> i32 {
        self.largeur as i32 * TILE_SIZE
    }

    // Obtenir la hauteur en pixels
    pub fn hauteur_pixels(&self) -> i32 {
        self.hauteur as i32 * TILE_SIZE
    }

    // Vérifier si une position en pixels est valide et pas bloquée
    pub fn peut_se_deplacer(&self, x: i32, y: i32) -> bool {
        // Convertir les pixels en indices de tile
        let tile_x = (x / TILE_SIZE) as usize;
        let tile_y = (y / TILE_SIZE) as usize;

        if let Some(tile) = self.get_tile(tile_x, tile_y) {
            // On peut marcher sur certains terrains
            match tile.terrain {
                TerrainType::Eau | TerrainType::Batiment1 | TerrainType::Batiment2 => false,
                _ => true, // Herbe, HauteHerbe, Chemin, Pharmacie, Dresseur = ok
            }
        } else {
            false // Hors limites
        }
    }
}
