// import des modules standards
use std::cell::Cell;
use std::io;

// import du module créé
use dessin_salon::salon::materiau::Carreau;
use dessin_salon::salon;

// Définition des constantes
const JOINT: i32 = 2; // épaisseur du joint, variable à définir ici
const TAILLE_CARREAU: i32 = 900; // taille du carreau, variable à définir ici
const QUANTITE_INITIALE: i32 = 60; // quantité initiale de carreaux, variable à définir ici

fn main() {
    let point_initial = Cell::new((860 + JOINT,8310 - JOINT));
    let point_depart_carrelage = Cell::new((860 + JOINT, 8310 - JOINT));

    // demander le nom du fichier à l'utilisateur
    println!("Dans quel fichier dessiner la pièce ? Ce fichier sera situé dans test > svg");
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Impossible de lire le nom du fichier");

    // initialiser et dessiner la zone
    let path = &("tests/svg/".to_owned() + &file_name + &".svg".to_owned());
    let mut zone = salon::initialiser(path);

    // initialisation des carreaux à positionner
    let carreaux = Carreau{
        position_init: point_initial,
        position: point_depart_carrelage,
        taille: TAILLE_CARREAU,
        quantite: Cell::new(QUANTITE_INITIALE),
        epaisseur_joint: JOINT,
    };

    // traitement
    println!("Quantité initiale de carreaux : {:?}", carreaux.quantite.get());
    zone.couvrir(&carreaux); // couverture avec des carreaux pleins et mise en évidence des trous à combler
    println!("---- Résultat pour des carreaux de {:?}mm avec des joints de {:?}mm ----", TAILLE_CARREAU, JOINT);
    println!("Quantité de carreaux après couverture : {:?}", carreaux.quantite.get());
    println!("Aire non couverte : {:?}m2", f64::from(zone.aire_libre.into_inner())/f64::from(1000000));
}