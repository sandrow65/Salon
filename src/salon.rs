use std::cell::RefCell;

use plotters::prelude::SVGBackend;
use plotters::prelude::{WHITE, BLUE, RED};

pub mod materiau;
pub mod definition;

pub fn initialiser(path: &String) -> definition::ZoneACouvrir {
    let mut cloisons_a_dessiner = Vec::new();
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("Maison SUD"),
        from: (860, 8310),
        to: (6370, 8670),
        color: WHITE,
    });
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("Maison OUEST"),
        from: (500,860),
        to: (860, 8670),
        color: WHITE,
    });
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("Maison NORD"),
        from: (500,500),
        to: (9380, 860),
        color: WHITE,
    });
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("Maison NORD-EST"),
        from: (9380,500),
        to: (9740, 860),
        color: WHITE,
    });
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("Maison EST 1"),
        from: (9380,860),
        to: (9740, 5890),
        color: WHITE,
    });
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("Maison EST 2"),
        from: (9540,5890),
        to: (9740, 11070),
        color: WHITE,
    });
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("Garage SUD"),
        from: (6570,10870),
        to: (9540, 11070),
        color: WHITE,
    });
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("Garage OUEST 1"),
        from: (6370,8310),
        to: (6570, 11070),
        color: WHITE,
    });
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("Cellier NORD"),
        from: (5460, 4060),
        to: (9380,4130),
        color: RED,
    });
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("Placard OUEST"),
        from: (5460,4130),
        to: (5530,4730),
        color: RED,
    });
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("Cellier OUEST"),
        from: (6410, 4130),
        to: (6480,5730),
        color: RED,
    });
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("WC NORD"),
        from: (4510, 7280),
        to: (6410,7350),
        color: RED,
    });
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("WC OUEST"),
        from: (4740, 7350),
        to: (4810,8310),
        color: RED,
    });
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("Garage NORD"),
        from: (6410, 5730),
        to: (9380,5890),
        color: BLUE,
    });
    cloisons_a_dessiner.push(definition::Cloison{
        name: String::from("Garage OUEST 2"),
        from: (6410,5730),
        to: (6570,8310),
        color: BLUE,
    });

    // définition de la zone
    let mut zone = definition::ZoneACouvrir{
        cloisons: cloisons_a_dessiner,
        backend: SVGBackend::new(path, (15000, 15000)),
        aire_libre: RefCell::new(0),
    };

    // dessin de la zone
    let draw_result = zone.dessiner();
    match draw_result {
        Ok(()) => (),
        Err(error) => panic!("Problème lors du dessin de la cloison {:?}", error),
    };
    zone
}