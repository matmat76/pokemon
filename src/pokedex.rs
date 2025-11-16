use crate::sprite_loader::{load_image_file, crop_sprite, remove_blue_background, Sprite};
use macroquad::texture::Texture2D;

/// Structure pour stocker un Pokémon avec son sprite
pub struct PokemonSprite {
    pub nom: String,
    pub sprite: Sprite,
}

/// Structure pour une seule frame (animation) d'un Pokémon
pub struct PokemonFrame {
    pub nom: String,
    pub frame: Sprite,
}

/// Structure pour un Pokémon rendu (données logiques + sprite affichable)
/// C'est le lien entre la couche logique et la couche graphique
pub struct RenderedPokemon {
    pub nom: String,
    pub texture: Texture2D,  // ← La texture GPU prête à afficher
    pub x: f32,              // ← Position sur la carte
    pub y: f32,
}

/// Charge les sprites des Pokémon individuels depuis texture/pokemon/
pub fn load_pokemon_sprites() -> Result<Vec<PokemonSprite>, String> {
    let mut pokemons: Vec<PokemonSprite> = Vec::new();

    // Liste des fichiers Pokémon à charger
    let pokemon_files = vec![
        ("salameche", "texture/pokemon/salameche.png"),
        ("carapuce", "texture/pokemon/carapuce.png"),
        ("florizarre", "texture/pokemon/florizarre.png"),
        ("celebi", "texture/pokemon/celebi.png"),
    ];

    for (nom, path) in pokemon_files {
        let image = load_image_file(path)?;
        let mut sprite = Sprite::new(image);
        remove_blue_background(&mut sprite.data);
        
        pokemons.push(PokemonSprite {
            nom: nom.to_string(),
            sprite,
        });
    }

    Ok(pokemons)
}

/// Extrait une seule FRAME d'un Pokémon (pour les animations)
/// frame_x, frame_y : position de la frame dans le sprite (0, 1, 2...)
/// frame_width, frame_height : taille de chaque frame
pub fn extract_pokemon_frame(
    pokemon_sprite: &PokemonSprite,
    frame_x: u32,
    frame_y: u32,
    frame_width: u32,
    frame_height: u32,
) -> Result<PokemonFrame, String> {
    let frame = crop_sprite(
        &pokemon_sprite.sprite.data,
        frame_x * frame_width,
        frame_y * frame_height,
        frame_width,
        frame_height,
    )?;

    Ok(PokemonFrame {
        nom: format!("{}_frame_{}_{}", pokemon_sprite.nom, frame_x, frame_y),
        frame: Sprite::new(frame),
    })
}

/// Crée une liste de Pokémon rendus avec leurs positions sur la carte
/// C'est la fonction de LIAISON entre backend (logique) et frontend (graphique)
pub fn create_rendered_pokemons(
    pokemon_sprites: &[PokemonSprite],
    positions: &[(f32, f32)],  // (x_tile, y_tile) positions sur la carte
    tile_size: f32,
) -> Vec<RenderedPokemon> {
    let mut rendered = Vec::new();

    for (i, sprite) in pokemon_sprites.iter().take(positions.len()).enumerate() {
        let (tile_x, tile_y) = positions[i];
        let texture = sprite.sprite.to_macroquad_texture();

        // Convertir les coordonnées tiles en pixels
        let x = tile_x * tile_size;
        let y = tile_y * tile_size;

        rendered.push(RenderedPokemon {
            nom: sprite.nom.clone(),
            texture,
            x,
            y,
        });
    }

    rendered
}
