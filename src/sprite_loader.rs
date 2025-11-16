/**
 * Author : copilot
 */
use image::{ImageReader, RgbaImage};

/// Charge une image depuis un fichier
pub fn load_image_file(path: &str) -> Result<RgbaImage, String> {
    let img = ImageReader::open(path)
        .map_err(|e| format!("Erreur lors de l'ouverture de {}: {}", path, e))?
        .decode()
        .map_err(|e| format!("Erreur lors du décodage de {}: {}", path, e))?;
    
    Ok(img.to_rgba8())
}

/// Extrait une région (rect) d'une image
/// x, y = position du coin supérieur gauche
/// width, height = dimensions du rectangle à extraire
pub fn crop_sprite(
    image: &RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<RgbaImage, String> {
    if x + width > image.width() || y + height > image.height() {
        return Err(format!(
            "Erreur: sprite hors limites. Image: {}x{}, Demandé: ({}, {}) de {}x{}",
            image.width(),
            image.height(),
            x,
            y,
            width,
            height
        ));
    }

    let mut cropped = RgbaImage::new(width, height);
    
    for dy in 0..height {
        for dx in 0..width {
            let pixel = image.get_pixel(x + dx, y + dy);
            cropped.put_pixel(dx, dy, *pixel);
        }
    }

    Ok(cropped)
}

/// Enlève le fond bleu Pokémon (le rend transparent)
/// La couleur bleu Pokemon est environ: RGB(72, 120, 200) ou similaire
pub fn remove_blue_background(image: &mut RgbaImage) {
    for pixel in image.pixels_mut() {
        let r = pixel.0[0] as f32;
        let g = pixel.0[1] as f32;
        let b = pixel.0[2] as f32;
        
        // Détecte le bleu Pokémon (B > R et B > G et B est dominant)
        // On utilise une tolérance pour capturer le bleu ET ses variations
        if b > r + 20.0 && b > g + 20.0 && b > 150.0 {
            // C'est du bleu → le rendre transparent
            pixel.0[3] = 0; // Alpha = 0 (invisible)
        }
    }
}

/// Convertit une RgbaImage en Macroquad Texture2D
pub fn rgba_to_macroquad_texture(image: &RgbaImage) -> macroquad::texture::Texture2D {
    let width = image.width() as u16;
    let height = image.height() as u16;
    
    // Convertir les pixels en format que Macroquad accepte
    let mut pixels = Vec::new();
    for pixel in image.pixels() {
        pixels.push(pixel.0[0]); // R
        pixels.push(pixel.0[1]); // G
        pixels.push(pixel.0[2]); // B
        pixels.push(pixel.0[3]); // A
    }

    // Créer la texture Macroquad
    macroquad::texture::Texture2D::from_rgba8(width, height, &pixels)
}

/// Structure pour stocker un sprite chargé
pub struct Sprite {
    pub data: RgbaImage,
    pub width: u32,
    pub height: u32,
}

impl Sprite {
    pub fn new(data: RgbaImage) -> Self {
        let width = data.width();
        let height = data.height();
        Sprite { data, width, height }
    }

    /// Convertit le sprite en texture Macroquad
    pub fn to_macroquad_texture(&self) -> macroquad::texture::Texture2D {
        rgba_to_macroquad_texture(&self.data)
    }
}
