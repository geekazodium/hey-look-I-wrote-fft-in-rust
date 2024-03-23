use std::f64::consts::PI;

use crate::complex_number::ComplexNum64;

pub fn ifft(points: &Vec<ComplexNum64>, ascending: bool) -> Vec<ComplexNum64> {
    if (points.len() as f64).log2().fract() != 0. {
        panic!("size of array must be power of 2");
    }
    let mut m: Vec<&ComplexNum64> = Vec::new();
    for p in points {
        m.push(p);
    }
    m.reverse();
    let mut m = ifft_recurse(m);
    let mul_ = ComplexNum64::new(1. / (m.len() as f64), 0.);
    if !ascending {
        m.reverse();
    }
    let mut ret = vec![];
    for ele in m {
        ret.push(ele.mul(&mul_));
    }
    return ret;
}

pub fn ifft_recurse(points: Vec<&ComplexNum64>) -> Vec<ComplexNum64> {
    if points.len() == 1 {
        let v = points.get(0).unwrap();
        return vec![ComplexNum64::new(v.getReal(), v.getImaginary())];
    }
    let mut even: Vec<&ComplexNum64> = Vec::with_capacity(points.len() >> 1);
    let mut odd: Vec<&ComplexNum64> = Vec::with_capacity(points.len() >> 1);
    let mut b = false;
    for element in points {
        if b {
            even.push(element);
        } else {
            odd.push(element);
        }
        b = !b;
    }
    let even = ifft_recurse(even);
    let odd = ifft_recurse(odd);
    let mut polynomial = vec![];
    for i in 0..even.len() {
        let rot = ComplexNum64::from_polar(1., -(i as f64) / (even.len() as f64) * PI);
        polynomial.push(even[i].sum(&odd[i].mul(&rot)));
    }
    for i in 0..even.len() {
        let rot = ComplexNum64::from_polar(1., -(i as f64) / (even.len() as f64) * PI);
        polynomial.push(even[i].sub(&odd[i].mul(&rot)));
    }
    return polynomial;
}

pub fn fft(polynomial: &Vec<ComplexNum64>, ascending: bool) -> Vec<ComplexNum64> {
    if (polynomial.len() as f64).log2().fract() != 0. {
        panic!("size of array must be power of 2");
    }
    let mut m: Vec<&ComplexNum64> = Vec::new();
    for p in polynomial {
        m.push(p);
    }
    if ascending {
        m.reverse();
    }
    return fft_recurse(m);
}

pub fn fft_recurse(polynomial: Vec<&ComplexNum64>) -> Vec<ComplexNum64> {
    if polynomial.len() == 1 {
        let v = polynomial.get(0).unwrap();
        return vec![ComplexNum64::new(v.getReal(), v.getImaginary())];
    }
    let mut even: Vec<&ComplexNum64> = Vec::with_capacity(polynomial.len() >> 1);
    let mut odd: Vec<&ComplexNum64> = Vec::with_capacity(polynomial.len() >> 1);
    let mut b = false;
    for element in polynomial {
        if b {
            even.push(element);
        } else {
            odd.push(element);
        }
        b = !b;
    }
    let even = fft_recurse(even);
    let odd = fft_recurse(odd);
    let mut polynomial = vec![];
    for i in 0..even.len() {
        let rot = ComplexNum64::from_polar(1., (i as f64) / (even.len() as f64) * PI);
        polynomial.push(even[i].sum(&odd[i].mul(&rot)));
    }
    for i in 0..even.len() {
        let rot = ComplexNum64::from_polar(1., (i as f64) / (even.len() as f64) * PI);
        polynomial.push(even[i].sub(&odd[i].mul(&rot)));
    }
    return polynomial;
}
