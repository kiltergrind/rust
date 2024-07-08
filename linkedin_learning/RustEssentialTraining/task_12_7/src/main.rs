#[derive(PartialEq)]
struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

/* YOUR CODE GOES HERE */
use std::fmt::{Display, Error, Formatter};

const GRAVITATIONAL_ACC: f64 = 9.81;

trait Altitude {
    fn altitude(&self) -> f64;
}

impl Altitude for Satellite {
    fn altitude(&self) -> f64 {
        GRAVITATIONAL_ACC * (self.velocity / GRAVITATIONAL_ACC).powf(2.0) / 2.0
    }
}

impl Display for Satellite {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, 
            "Satellite {{\n\tname: {},\n\tvelocity: {},\n\taltitude: {},\n}}", 
            self.name,
            self.velocity,
            self.altitude())
    }
}

impl PartialOrd for Satellite {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let alt_gt = self.altitude() > other.altitude();
        let vel_lt = self.velocity < other.velocity;

        Some(match alt_gt && vel_lt {
            true => std::cmp::Ordering::Greater,
            false => std::cmp::Ordering::Less,
        })
    }
}


fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    let mubble = Satellite {
        name: String::from("Mubble Tetrascope"),
        velocity: 32.72
    };

    println!("hubble is {}", hubble);
    println!("mubble is {}", mubble);

    if hubble > mubble {
        println!("hubble is greater than mubble");
    } else if hubble < mubble {
        println!("mubble is greater than hubble");
    }
}
