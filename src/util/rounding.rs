pub fn rounding_dividing_f64(num1: f64, num2: f64) -> f64 {
    (num1 / num2 * 1000.0).round() / 1000.0
}

pub fn rounding_multiplication_f64(num1: f64, num2: f64) -> f64 {
    (num1 * num2 * 1000.0).round() / 1000.0
}