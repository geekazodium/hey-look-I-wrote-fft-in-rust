use std::ops::{Add, Mul};

use complex_number::ComplexNum64;
mod complex_number;

use fast_fourier_transform::{fft, ifft};
mod fast_fourier_transform;

fn main() {
    let polynomial = vec![
        ComplexNum64::new(9., 0.),
        ComplexNum64::new(1., 2.),
        ComplexNum64::new(3., 0.),
        ComplexNum64::new(2., 1.),
        ComplexNum64::zero(),
        ComplexNum64::zero(),
        ComplexNum64::zero(),
        ComplexNum64::zero()
    ];
    let polynomial2 = vec![
        ComplexNum64::new(1., 0.),
        ComplexNum64::new(1., 4.),
        ComplexNum64::new(3., 0.),
        ComplexNum64::new(-2., 9.),
        ComplexNum64::zero(),
        ComplexNum64::zero(),
        ComplexNum64::zero(),
        ComplexNum64::zero()
    ];
    for p in &polynomial {
        println!("{:?}", p);
    }
    println!();
    let points = fft(&polynomial, true);
    let points2 = fft(&polynomial2, true);
    let mut v = vec![];

    for i in 0..points.len() {
        v.push(points.get(i).unwrap().mul(points2.get(i).unwrap()));
    }

    println!();
    let new_polynomial = ifft(&v, true);
    for p in 0..new_polynomial.len() {
        println!(
            "{:?} + {:?}i",
            new_polynomial[p].getReal().mul(64.).round().mul(1./64.),
            new_polynomial[p].getImaginary().mul(64.).round().mul(1./64.)
        );
    }
}

fn sample(polynomial: &Vec<ComplexNum64>, point: &ComplexNum64) -> ComplexNum64 {
    let mut v = ComplexNum64::zero();
    for i in Iterator::rev(0..polynomial.len()) {
        v = v.mul(point);
        v = v.sum(polynomial.get(i).unwrap());
    }
    return v;
}
