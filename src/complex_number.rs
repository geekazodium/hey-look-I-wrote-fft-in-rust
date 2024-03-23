#[derive(Debug)]
pub struct ComplexNum64 {
    re: f64,
    im: f64,
}

impl ComplexNum64 {
    pub fn mul(&self, other: &ComplexNum64) -> ComplexNum64 {
        ComplexNum64 {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
    pub fn sum(&self, other: &ComplexNum64) -> ComplexNum64 {
        ComplexNum64 {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
    pub fn sub(&self, other: &ComplexNum64) -> ComplexNum64 {
        ComplexNum64 {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
    pub fn new(re: f64, im: f64) -> ComplexNum64 {
        return ComplexNum64 { re, im };
    }
    pub fn from_polar(magnitue: f64, argument: f64) -> ComplexNum64 {
        return ComplexNum64 {
            re: magnitue * f64::cos(argument),
            im: magnitue * f64::sin(argument),
        };
    }
    pub fn zero() -> ComplexNum64 {
        return ZERO;
    }
    pub fn getReal(&self) -> f64 {
        return self.re;
    }
    pub fn getImaginary(&self) -> f64 {
        return self.im;
    }
}

const ZERO: ComplexNum64 = ComplexNum64 { re: 0., im: 0. };