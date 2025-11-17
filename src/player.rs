// Direction du joueur
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// État d'animation
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationState {
    RunningLeft,
    RunningRight,
}

// Représente le joueur sur la map
pub struct Player {
    pub nom: String,
    pub x: i32,  // Position en pixels (pas en indices)
    pub y: i32,  // Position en pixels (pas en indices)
    pub direction: Direction,  // Direction actuelle
    pub animation_state: AnimationState,  // État d'animation
    pub animation_timer: f32,  // Compte les secondes pour l'animation
    pub movement_cooldown: f32,  // Temps avant le prochain mouvement
}

impl Player {
    pub fn new(nom: String, x: i32, y: i32) -> Self {
        Player { 
            nom, 
            x, 
            y,
            direction: Direction::Down,  // Direction par défaut
            animation_state: AnimationState::RunningLeft,  // Animation au démarrage
            animation_timer: 0.0,
            movement_cooldown: 0.0,  // Pas de cooldown au démarrage
        }
    }
    // Mettre à jour le timer d'animation (appeler chaque frame avec delta_time)
    pub fn update_animation(&mut self, delta_time: f32) {
        self.animation_timer += delta_time;
        
        // Réinitialiser le timer toutes les 0.5 secondes
        if self.animation_timer >= 0.5 {
            self.animation_timer = 0.0;
            
            // Alterner entre RunningLeft et RunningRight si en mouvement
            if self.animation_state == AnimationState::RunningLeft {
                self.animation_state = AnimationState::RunningRight;
            } else if self.animation_state == AnimationState::RunningRight {
                self.animation_state = AnimationState::RunningLeft;
            }
        }
    }
    // Mettre à jour le cooldown de mouvement
    pub fn update_movement_cooldown(&mut self, delta_time: f32) {
        if self.movement_cooldown > 0.0 {
            self.movement_cooldown -= delta_time;
        }
    }
    
    // Vérifier si on peut se déplacer (cooldown écoulé?)
    pub fn can_move(&self) -> bool {
        self.movement_cooldown <= 0.0
    }
    // Réinitialiser le cooldown après un mouvement
    fn reset_movement_cooldown(&mut self) {
        self.movement_cooldown = 0.01; // 50ms entre chaque mouvement (très rapide)
    }
    
    // Obtenir le nom de la frame actuelle
    pub fn get_frame_name(&self) -> String {
        let direction_name = match self.direction {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        };
        
        let animation_name = match self.animation_state {
            AnimationState::RunningLeft => "running_left",
            AnimationState::RunningRight => "running_right",
        };
        
        format!("trainer_{}_{}", direction_name, animation_name)
    }

    pub fn move_right(&mut self, largeur_max: i32) {
        let nouvelle_x = self.x + 1;
        if nouvelle_x < largeur_max {
            self.x = nouvelle_x;
        }
        self.direction = Direction::Right;
        self.animation_state = AnimationState::RunningLeft; 
        self.reset_movement_cooldown();
    }

    pub fn move_left(&mut self) {
        let nouvelle_x = self.x - 1;
        if nouvelle_x >= 0 {
            self.x = nouvelle_x;
        }
        self.direction = Direction::Left;
        self.animation_state = AnimationState::RunningLeft;
        self.reset_movement_cooldown();
    }

    pub fn move_down(&mut self, hauteur_max: i32) {
        let nouvelle_y = self.y + 1;
        if nouvelle_y < hauteur_max {
            self.y = nouvelle_y;
        }
        self.direction = Direction::Down;
        self.animation_state = AnimationState::RunningLeft;
        self.reset_movement_cooldown();
    }

    pub fn move_up(&mut self) {
        let nouvelle_y = self.y - 1;
        if nouvelle_y >= 0 {
            self.y = nouvelle_y;
        }
        self.direction = Direction::Up;
        self.animation_state = AnimationState::RunningLeft;
        self.reset_movement_cooldown();
    }

    pub fn get_tile_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
