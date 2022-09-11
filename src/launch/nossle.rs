use uom::si::f32::*;
use uom::si::length::meter;
use uom::si::mass::kilogram;
use uom::typenum::P2;
use std::f32::consts::PI;

pub struct Nossle {
    area: Area,
    mass: Mass,
    radius: Length,
}

pub enum NossleType {
    WaterNossle
}

impl Nossle {
    pub fn new(nossle_type: NossleType) -> Nossle {
        let (radius, mass) = match nossle_type {
            NossleType::WaterNossle=>(
                Length::new::<meter>(0.021/2.),
                Mass::new::<kilogram>(24./1000.)
                )
        };
        let area = PI * radius.powi(P2::new());
        Nossle {
            area,
            mass,
            radius
        }
    }
}
