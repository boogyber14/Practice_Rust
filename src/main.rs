use nalgebra::{DMatrix, Vector1};

// Define a function f(x) = x^2
fn f(x: f64) -> f64 {
    x.powi(2)
}

// Numerical differentiation to calculate the derivative of f(x) at a point
fn numerical_differentiation(x: f64, h: f64) -> f64 {
    let df = (f(x + h) - f(x)) / h;
    df
}

fn main() {
    let x = 2.0; // Point at which to calculate the derivative
    let h = 0.0001; // Step size for numerical differentiation
    let df = numerical_differentiation(x, h);
    println!("Derivative of f(x) at x = {}: {}", x, df);
}
