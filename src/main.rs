use core::fmt;
use std::{cmp::Ordering, fmt::Display};

struct Equation {
    a: f64,
    b: f64,
    c: f64,
}

impl Equation {
    fn from(input: [f64; 3]) -> Equation {
        Equation {
            a: input[0],
            b: input[1],
            c: input[2],
        }
    }

    fn solve(&self) -> Solution {
        let delta: f64 = self.b.powi(2) - 4.0 * self.a * self.c;
        match delta.partial_cmp(&0.0) {
            Some(Ordering::Less) => {
                return Solution {
                    first: None,
                    second: None,
                }
            }
            _ => {
                return Solution {
                    first: Some((-self.b - (delta as f64).sqrt()) / 2.0 * self.a),
                    second: Some((-self.b + (delta as f64).sqrt()) / 2.0 * self.a),
                }
            }
        }
    }
}

struct Solution {
    first: Option<f64>,
    second: Option<f64>,
}

impl Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})*x^2 + ({})*x + ({})", self.a, self.b, self.c)
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tup = (self.first, self.second);
        match tup {
            (Some(t), Some(i)) => write!(f, "2 solutions are {} and {}", t, i),
            (None, None) => write!(f, "no solutions here for you nigga"),
            _ => write!(f, "idk"),
        }
    }
}

fn main() {
    let eq = Equation::from([1.0, -8.0, 15.0]);
    println!("{eq}");
    println!("solutions {}", eq.solve());

    let eq = Equation::from([4.0, 4.0, 4.0]);
    println!("{eq}");
    println!("solutions {}", eq.solve());

    let eq = Equation::from([-1.0, -8.0, 15.0]);
    println!("{eq}");
    println!("solutions {}", eq.solve());

    let eq = Equation::from([2.9, 4.0, 3889.9]);
    println!("{eq}");
    println!("solutions {}", eq.solve());
}
