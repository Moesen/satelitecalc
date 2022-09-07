extern crate uom;

use uom::si::f32::*;
use uom::si::length;
use uom::si::mass::kilogram;
use uom::si::mass_concentration::kilogram_per_cubic_meter;
use uom::si::volume::cubic_meter;
use uom::si::area;

pub struct Propellant {
    density: MassConcentration,
    initial_volume: Volume,
    mass: Mass,
}

pub struct Nossle {
    radius: Length,
    area: Area,
    mass: Mass,
}

pub struct Rocket {
    area: Area,
    radius: Length,
    nossle: Nossle,
    mass: Mass,
    volume: Volume,
    propellant: Propellant,
}

fn main() {
    let p = Propellant {
        density: MassConcentration::new::<kilogram_per_cubic_meter>(5.0), 
        initial_volume: Volume::new::<cubic_meter>(1.0), 
        mass: Mass::new::<kilogram>(1.0)
    };
    println!("{:?}", p.mass);
    println!("{:?}", p.initial_volume);
    println!("{:?}", p.density);
    println!("{:?}", (p.mass/p.density));
    
}
