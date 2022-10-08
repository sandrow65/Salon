use std::cell::Cell;
use plotters_backend::BackendCoord;

use crate::salon::definition::Cloison;

pub struct Carreau {
    pub position_init: Cell<BackendCoord>,
    pub position: Cell<BackendCoord>,
    pub taille: i32,
    pub quantite: Cell<i32>,
}

impl Carreau {
    pub fn croise_cloison(&self, liste_cloisons: &Vec<Cloison>) -> (bool, Option<i32>, Option<i32>) {
        let mut croise = false;
        let mut dist_libre_x = None;
        let mut dist_libre_y = None;
        for cloison in liste_cloisons.iter() {
            if !(self.position.get().0 >= cloison.to.0 || self.position.get().0 + self.taille <= cloison.from.0 || self.position.get().1 - self.taille >= cloison.to.1 || self.position.get().1 <= cloison.from.1)
                {
                    println!("Croise la cloison {:?}", cloison.name);
                    croise = true;
                    if cloison.from.0 - self.position.get().0 < 0 {
                        dist_libre_x = Some(cloison.to.0 - self.position.get().0);
                        dist_libre_y = Some(self.position.get().1 - cloison.to.1);
                        break;
                    }
                    else {
                        dist_libre_x = Some(cloison.from.0 - self.position.get().0);
                    }
                    if self.position.get().1 - cloison.from.1 < 0 {
                        dist_libre_y = Some(self.position.get().1 - cloison.to.1);
                    }
                    else {
                        dist_libre_y = Some(self.position.get().1 - cloison.from.1);
                    }
                    break;
                }
        }
        (croise, dist_libre_x, dist_libre_y)
    }
}