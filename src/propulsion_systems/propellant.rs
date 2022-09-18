use uom::si::{f32::*, mass_density::kilogram_per_cubic_meter};
pub struct Propellant {
    density: MassDensity,
}

pub enum PropellantType {
    Water,
}

impl Propellant {
    pub fn new(propellant_type: PropellantType) -> Propellant {
        match propellant_type {
            PropellantType::Water => Propellant {
                density: MassDensity::new::<kilogram_per_cubic_meter>(998.),
            },
        }
    }
}
