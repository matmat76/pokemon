/**
 * Author : Copilot
 */
use macroquad::texture::Texture2D;
use std::collections::HashMap;

/// Structure pour stocker toutes les animations du dresseur
pub struct TrainerAnimations {
    pub frames: HashMap<String, Texture2D>,
}

impl TrainerAnimations {
    /// Charger toutes les 12 frames du dresseur
    pub async fn load() -> Result<Self, String> {
        let mut frames = HashMap::new();

        // Directions: up, down, left, right
        // Animations: stop, running_left, running_right
        let directions = ["up", "down", "left", "right"];
        let animations = ["stop", "running_left", "running_right"];

        for direction in &directions {
            for animation in &animations {
                let filename = format!("texture/dresseur/trainer_{}_{}.png", direction, animation);
                
                match macroquad::texture::load_texture(&filename).await {
                    Ok(texture) => {
                        let key = format!("trainer_{}_{}", direction, animation);
                        frames.insert(key, texture);
                    }
                    Err(e) => {
                        return Err(format!("Erreur lors du chargement de {}: {}", filename, e));
                    }
                }
            }
        }

        Ok(TrainerAnimations { frames })
    }

    /// Obtenir la frame correspondant à un nom
    pub fn get_frame(&self, frame_name: &str) -> Option<&Texture2D> {
        self.frames.get(frame_name)
    }
}
