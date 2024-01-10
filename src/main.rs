// Write a program that calculates the BMI
// Inputs: height inches, weight lbs.
// Process: bmi = (weight / (height × height))* 703
// Output: if bmi < 18.5, underweight, > 25 overweight, in between is normal.
// Your BMI is {}.
// "You are underweight. You should see your doctor." | "You are within the ideal weight range." | "You are overweight. You should see your doctor."

fn round_decimal(number: f64, place: i32) -> f64 {
    let multiplier: f64 = 10_f64.powi(place);
    (number * multiplier).round() / multiplier
}

fn calculate_bmi(height: f64, weight: f64) -> f64 {
    let bmi: f64 = (weight / (height * height)) * 703.0;
    // round to 2 decimal digits
    round_decimal(bmi, 2)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_bmi() {
        assert_eq!(calculate_bmi(72.0, 154.0), 20.88);
        assert_eq!(calculate_bmi(64.0, 140.0), 24.03);
        assert_eq!(calculate_bmi(69.0, 155.0), 22.89);
        assert_eq!(calculate_bmi(71.0, 165.0), 23.01);
        assert_eq!(calculate_bmi(70.0, 120.0), 17.22);
        assert_eq!(calculate_bmi(66.0, 180.0), 29.05);
    }
}
fn main() {
    println!("Hello, world!");
}
