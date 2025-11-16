use crate::pokemon::Pokemon;
use macroquad::prelude::{Texture2D, DrawTextureParams, Vec2, WHITE, draw_texture_ex};

pub struct PokemonRenderable {
    pokemon: Box<dyn Pokemon>,
    texture: Texture2D,
    x: f32,
    y: f32,
}


impl PokemonRenderable {
    pub fn new(pokemon: Box<dyn Pokemon>, texture: Texture2D, x_init: f32, y_init: f32) -> PokemonRenderable {
        PokemonRenderable {
            pokemon,
            texture,
            x: x_init,
            y: y_init,
        }
    }

    pub fn afficher(&self) {
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(32.0, 32.0)),
            ..Default::default()
        };
        draw_texture_ex(&self.texture, self.x, self.y, WHITE, params);
    }
    
    pub fn get_pokemon(&self) -> &dyn Pokemon{
        &*self.pokemon
    }
    
    pub fn get_pokemon_mut(&mut self) -> &mut dyn Pokemon {
        &mut *self.pokemon
    }
    
    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    
    pub fn get_hitbox(&self) -> (f32, f32, f32, f32) {
        // Retourne (x, y, largeur, hauteur) de la hitbox
        (self.x, self.y, 32.0, 32.0)
    }
}