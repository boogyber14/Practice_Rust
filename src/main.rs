// Define a struct to represent a polynomial function
struct Polynomial {
    coeffs: Vec<f64>,
}

impl Polynomial {
    // Method to evaluate the polynomial at a given point
    fn evaluate(&self, x: f64) -> f64 {
        let mut result = 0.0;
        for (i, coeff) in self.coeffs.iter().enumerate() {
            result += coeff * x.powi(i as i32);
        }
        result
    }

    // Method to differentiate the polynomial
    fn differentiate(&self) -> Polynomial {
        let mut new_coeffs = Vec::new();
        for (i, coeff) in self.coeffs.iter().enumerate().skip(1) {
            new_coeffs.push(*coeff * (i as f64));
        }
        Polynomial { coeffs: new_coeffs }
    }
}

fn main() {
    // Create a polynomial x^2 + 2x + 1
    let poly = Polynomial { coeffs: vec![1.0, 2.0, 1.0] };

    // Evaluate the polynomial at x = 2
    let result = poly.evaluate(2.0);
    println!("f(2) = {}", result);

    // Differentiate the polynomial
    let derivative = poly.differentiate();
    println!("f'(x) = {:?}", derivative.coeffs);
}
