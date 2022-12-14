use std::cell::RefCell;
use plotters::prelude::RGBColor;
use plotters_backend::{BackendColor, BackendCoord, DrawingBackend};
use plotters::style::text_anchor::{Pos, HPos, VPos};
use plotters::prelude::SVGBackend;
use plotters::prelude::{GREEN, MAGENTA};
use plotters::prelude::{FontStyle, FontFamily, FontTransform};
use plotters::style::{TextStyle, FontDesc};

use crate::salon::materiau::Carreau;

const MIN: i32 = 860;
const MAX: i32 = 10870;
const TEXT_COLOR: BackendColor =  BackendColor{ // WHITE
    alpha: 100.0,
    rgb: (255,255,255),
};

pub struct Cloison {
    pub name: String,
    pub from: BackendCoord,
    pub to: BackendCoord,
    pub color: RGBColor,
}

pub struct ZoneACouvrir<'a> {
    pub backend: SVGBackend<'a>,
    pub cloisons: Vec<Cloison>,
    pub aire_libre: RefCell<i32>,
}

impl<'a> ZoneACouvrir<'a> {
    pub fn couvrir(&mut self, carreaux: &Carreau) {
        let mut counter = 1;
        loop {
            println!("Position actuelle : {:?}", carreaux.position.get());
            let (croise_cloison, dist_libre_x, dist_libre_y) = carreaux.croise_cloison(&self.cloisons);
            if croise_cloison {
                println!("Distance x : {:?}", dist_libre_x);
                println!("Distance y : {:?}", dist_libre_y);
                let upper_left = if let Some(y) = dist_libre_y {
                    (carreaux.position.get().0, carreaux.position.get().1 - y)
                } else {
                    (carreaux.position.get().0, carreaux.position.get().1 - carreaux.taille)
                };
                // match dist_libre_y {
                //     Some(y) if y < carreaux.taille => (carreaux.position.get().0, carreaux.position.get().1 - y), //
                //     None | _ => (carreaux.position.get().0, carreaux.position.get().1 - carreaux.taille),
                // };
                let bottom_right = if let Some(x) = dist_libre_x {
                    (carreaux.position.get().0 + x , carreaux.position.get().1)
                } else {
                    (carreaux.position.get().0 + carreaux.taille, carreaux.position.get().1)
                };
                // match dist_libre_x {
                //     Some(x) if x < carreaux.taille => (carreaux.position.get().0 + x , carreaux.position.get().1), // 
                //     None | _ => (carreaux.position.get().0 + carreaux.taille, carreaux.position.get().1),
                // };
                // dessiner le rectangle qui reste libre apr??s avoir fini la ligne
                let draw_result = self.backend.draw_rect(
                    upper_left,
                    bottom_right,
                    &GREEN,
                    true
                );
                // prendre en compte les erreurs ??ventuelles
                match draw_result {
                    Ok(()) => (),
                    Err(error) => panic!("Probl??me lors du trac?? de l'aire libre: {:?}", error),
                };
                // ??crire la distance libre en horizontal
                let draw_result = self.backend.draw_text(
                    &("x = ".to_owned() + &(dist_libre_x.unwrap() / 10).to_string() + &"cm".to_owned()),
                    &TextStyle{
                        font: FontDesc::new(FontFamily::SansSerif, 50.0, FontStyle::Oblique),
                        color: TEXT_COLOR,
                        pos: Pos::new(HPos::Center, VPos::Center),
                    },
                    (upper_left.0 + (bottom_right.0 - upper_left.0)/2, bottom_right.1 - (bottom_right.1 - upper_left.1)/4)
                );
                match draw_result {
                    Ok(()) => (),
                    Err(error) => panic!("Probl??me lors de l'??criture de la distance hozizontale libre: {:?}", error),
                };
                // ??crire la distance libre en vertical
                let draw_result = self.backend.draw_text(
                    &("y = ".to_owned() + &(dist_libre_y.unwrap() / 10).to_string() + &"cm".to_owned()),
                    &TextStyle{
                        font: FontDesc::new(FontFamily::SansSerif, 50.0, FontStyle::Oblique).transform(FontTransform::Rotate270),
                        color: TEXT_COLOR,
                        pos: Pos::new(HPos::Center, VPos::Center),
                    },
                    (upper_left.0 + 3*(bottom_right.0 - upper_left.0)/4, bottom_right.1 - (bottom_right.1 - upper_left.1)/2)
                );
                match draw_result {
                    Ok(()) => (),
                    Err(error) => panic!("Probl??me lors de l'??criture de la distance verticale libre: {:?}", error),
                };

                carreaux.position.set((carreaux.position_init.get().0, carreaux.position_init.get().1 - counter*(carreaux.taille + carreaux.epaisseur_joint)));
                counter += 1;
                let aire = match dist_libre_x {
                    Some(x) if x < carreaux.taille => match dist_libre_y {
                        Some(y) if y < carreaux.taille => x * y,
                        None | _ => x * carreaux.taille,
                    },
                    None | _ => match dist_libre_y {
                        Some(y) if y < carreaux.taille => carreaux.taille * y,
                        None | _ => carreaux.taille * carreaux.taille,
                    }
                };
                self.aire_libre.replace_with(|&mut old| old + aire);
            }
            else {
                carreaux.quantite.set(carreaux.quantite.get() - 1);
                let draw_result = self.backend.draw_rect(
                    (carreaux.position.get().0, carreaux.position.get().1 - carreaux.taille), 
                    (carreaux.position.get().0 + carreaux.taille, carreaux.position.get().1), 
                    &MAGENTA, 
                    true
                );
                match draw_result {
                    Ok(()) => (),
                    Err(error) => panic!("Probl??me lors du dessin du carreau {:?}", error),
                };
                carreaux.position.set((carreaux.position.get().0 + carreaux.taille + carreaux.epaisseur_joint, carreaux.position.get().1));
            }

            if carreaux.position.get().1 < MIN || carreaux.position.get().0 > MAX {
                break;
            }
        }
    }
}    

impl<'a> ZoneACouvrir<'a> {
    pub fn dessiner (&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create a 800*600 bitmap and start drawing
        
        for cloison in self.cloisons.iter() {
            let draw_result = self.backend.draw_rect(
                cloison.from, 
                cloison.to, 
                &cloison.color, 
                true);
            match draw_result {
                Ok(()) => (),
                Err(error) => panic!("Probl??me lors du dessin de la cloison {:?}", error),
            };
        }
        
        Ok(())
    }   
}