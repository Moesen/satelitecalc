use super::nossle;
use super::propellant;

use std::f32::consts::PI;
use uom::si::f32::*;
use uom::si::length::meter;
use uom::si::mass::kilogram;
use uom::si::volume::cubic_meter;
use uom::typenum::P2;

pub struct Rocket {
    area: Area,                         // Calc
    mass: Mass,                         // Given
    nossle: nossle::Nossle,             //Set
    propellant: propellant::Propellant, //Set
    radius: Length,                     // Given
    volume: Volume,                     // Given
}

pub enum RocketType {
    WaterRocketMedium,
}

impl Rocket {
    pub fn new(
        rocket_type: RocketType,
        nossle_type: nossle::NossleType,
        propellant_type: propellant::PropellantType,
    ) -> Rocket {
        let nossle = nossle::Nossle::new(nossle_type);
        let propellant = propellant::Propellant::new(propellant_type);
        let (radius, mass, volume) = match rocket_type {
            RocketType::WaterRocketMedium => (
                Length::new::<meter>(0.065 / 2.),
                Mass::new::<kilogram>(38.7 / 1000.),
                Volume::new::<cubic_meter>(0.9 / 1000.),
            ),
        };
        let area = PI * radius.powi(P2::new());

        Rocket {
            area,
            radius,
            nossle,
            mass,
            volume,
            propellant,
        }
    }
}
