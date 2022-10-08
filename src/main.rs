// import des modules standards
use std::cell::Cell;
use std::cell::RefCell;

// import du module crééé
use dessin_salon::salon::definition;
use dessin_salon::salon::ZoneACouvrir;
use dessin_salon::salon::materiau::Carreau;

// Définition des constantes
const JOINT: i32 = 2; // épaisseur du joint, variable à définir ici
const TAILLE_CARREAU: i32 = 900; // taille du carreau, variable à définir ici
const QUANTITE_INITIALE: i32 = 60; // quantité initiale de carreaux, variable à définir ici

fn main() {
    let point_initial = Cell::new((860 + JOINT,8310 - JOINT));
    let point_depart_carrelage = Cell::new((860 + JOINT, 8310 - JOINT));
    // dessin de la zone
    let res_dessin = definition::dessiner();
    let backend = res_dessin.0.unwrap();
    let cloisons = res_dessin.1;

    // définition de la zone
    let mut zone = ZoneACouvrir{
        backend: backend,
        aire_libre: RefCell::new(0),
    };
    
    // initialisation des carreaux à positionner
    let carreaux = Carreau{
        position_init: point_initial,
        position: point_depart_carrelage,
        taille: TAILLE_CARREAU,
        quantite: Cell::new(QUANTITE_INITIALE),
    };

    // traitement
    println!("Quantité initiale de carreaux : {:?}", carreaux.quantite.get());
    zone.couvrir(&carreaux, cloisons, JOINT); // couverture avec des carreaux pleins et mise en évidence des trous à combler
    println!("Quantité de carreaux après couverture : {:?}", carreaux.quantite.get());
    println!("Aire non couverte : {:?}m2", f64::from(zone.aire_libre.into_inner())/f64::from(1000000));
}