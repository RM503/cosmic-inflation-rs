// Define various potentials and their first and second derivations

pub trait Potential {
    fn value(&self, x: f64) -> f64;
    fn gradient(&self, x: f64) -> f64;
    fn curvature(&self, x: f64) -> f64;
}


#[derive(Debug, Clone, Copy)]
pub struct Quadratic;

impl Potential for Quadratic {
    fn value(&self, x: f64) -> f64 {
        0.5 * x * x
    }

    fn gradient(&self, x: f64) -> f64 {
        x
    }

    fn curvature(&self, x: f64) -> f64 {
        1.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Starobinsky;

impl Potential for Starobinsky {
    fn value(&self, x: f64) -> f64 {
        0.75 * (1.0 - (-x).exp()) * (1.0 - (-x).exp())
    }

    fn gradient(&self, x: f64) -> f64 {
        1.5 * (1.0 - (-x).exp()) * (-x).exp()
    }

    fn curvature(&self, x: f64) -> f64 {
        1.5 * (-x).exp() * (2.0 * (-x).exp() - 1.0)
    }
}