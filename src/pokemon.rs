pub trait Pokemon {
    // Méthode pour attaquer
    fn attaquer(&self) -> i32;

    // Méthode pour prendre des dégâts
    fn prendre_degats(&mut self, degats: i32);

    // Méthode pour vérifier s'il est vivant
    fn est_vivant(&self) -> bool;

    // Méthode pour afficher les infos
    fn afficher_infos(&self);

    // Pour récupérer le nom du Pokémon 
    fn get_nom(&self) -> &String;

    // Pour afficher les PV restants
    fn get_pv(&self) -> i32;
}
#[derive(Debug, Clone, Copy)]
pub enum Type{
    Feu,
    Eau,
    Herbe,
}

pub struct Flamby{
    pub nom: String,
    vie: i32,
    attaque: i32, 
    type_element: Type,
}

pub struct Aquali{
    pub nom: String,
    vie: i32,
    attaque: i32, 
    type_element: Type,
}

pub struct Florizarre{
    pub nom: String,
    vie: i32,
    attaque: i32, 
    type_element: Type,
}

impl Pokemon for Flamby {
    fn attaquer(&self) -> i32 {
        self.attaque
    }

    fn prendre_degats(&mut self, degats: i32) {
        self.vie -= degats;
    }

    fn est_vivant(&self) -> bool{
        self.vie > 0
    }

    fn afficher_infos(&self) {
        println!("{} (Type: {:?}) - PV {} | Attaque {}", self.nom, self.type_element, self.vie, self.attaque);
    }
    fn get_nom(&self) -> &String {
        &self.nom
    }
    fn get_pv(&self) -> i32 {
        self.vie
    }
}

impl Pokemon for Aquali {
    fn attaquer(&self) -> i32 {
        self.attaque
    }

    fn prendre_degats(&mut self, degats: i32) {
        self.vie -= degats;
    }

    fn est_vivant(&self) -> bool{
        self.vie > 0
    }

    fn afficher_infos(&self) {
        println!("{} (Type: {:?}) - PV {} | Attaque {}", self.nom, self.type_element, self.vie, self.attaque);
    }
    fn get_nom(&self) -> &String {
        &self.nom
    }
    fn get_pv(&self) -> i32 {
        self.vie
    }
}

impl Pokemon for Florizarre {
    fn attaquer(&self) -> i32 {
        self.attaque
    }

    fn prendre_degats(&mut self, degats: i32) {
        self.vie -= degats;
    }

    fn est_vivant(&self) -> bool{
        self.vie > 0
    }

    fn afficher_infos(&self) {
        println!("{} (Type: {:?}) - PV {} | Attaque {}", self.nom, self.type_element, self.vie, self.attaque);
    }
    fn get_nom(&self) -> &String {
        &self.nom
    }
    fn get_pv(&self) -> i32 {
        self.vie
    }
}

impl Flamby{
    pub fn new(nom: String) -> Flamby {
        Flamby{
            nom,
            vie: 50,
            attaque: 15,
            type_element: Type::Feu,
        }
    }
}

impl Aquali{
    pub fn new(nom: String) -> Aquali {
        Aquali{
            nom,
            vie: 70,
            attaque: 10,
            type_element: Type::Eau,
        }
    }
}

impl Florizarre{
    pub fn new(nom: String) -> Florizarre {
        Florizarre{
            nom,
            vie: 60,
            attaque: 12,
            type_element: Type::Herbe,
        }
    }
}

pub fn calculer_efficacite(attaque_type: Type, defence_type: Type) -> f32 {
    match(attaque_type, defence_type){
        (Type::Feu, Type::Herbe) => 1.5,
        (Type::Herbe, Type::Eau) => 1.5,    
        (Type::Eau, Type::Feu) => 1.5,      
        (Type::Feu, Type::Eau) => 0.5,      
        (Type::Herbe, Type::Feu) => 0.5,    
        (Type::Eau, Type::Herbe) => 0.5,    
        _ => 1.0,  
    }
}